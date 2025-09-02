use anyhow::{Context, anyhow, bail};
use axum::{
    body::{self, Bytes},
    extract::{Multipart, Path as RestPath, State},
    http::{HeaderMap, HeaderName, StatusCode, header},
    response::IntoResponse,
};
use axum_typed_multipart::TypedMultipart;
use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult, SharedStr, utils::id_util::generate_uuid};
use futures::Stream;
use log::{info, warn};
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
    task::spawn_blocking,
};
use tokio_util::io::ReaderStream;

use crate::{
    ShareAppState,
    config::AttachmentConfig,
    controller::KResponse,
    krate::kfile::{
        KFileChunkUploadReq, KFileMeta, KFileUploadRsp, QueryKFileReq, mapper::KFileMapper,
    },
    util::digestutil::file_blake3_sum,
};

pub(crate) fn asset_tmp_path(config: &AttachmentConfig, upload_id: &str) -> PathBuf {
    std::path::Path::new(&config.base_dir.as_ref())
        .join("tmp-chunks")
        .join(upload_id)
}

fn assemble_file_sync<P: AsRef<Path> + Send>(
    temp_dir: P,
    config: AttachmentConfig,
    total_chunks: usize,
) -> AResult<String> {
    let output_filepath = temp_dir.as_ref().join(generate_uuid());
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&output_filepath)?;
    let mut hasher = blake3::Hasher::new();

    for chunk_number in 0..total_chunks {
        let chunk_path = temp_dir.as_ref().join(chunk_number.to_string());
        let chunk_data = std::fs::read(&chunk_path)?;
        hasher.write_all(&chunk_data)?;
        output_file.write_all(&chunk_data)?;
    }

    let sid = hasher.finalize().to_string();
    let path = config.get_sid_path(sid.as_str());
    std::fs::create_dir_all(path.parent().ok_or(anyhow!("unable to get parent"))?)?;

    std::fs::rename(output_filepath, path)?;
    // Clean up the temporary chunks
    std::fs::remove_dir_all(temp_dir)?;

    Ok(sid)
}

async fn assemble_file<P: AsRef<Path> + Send + 'static>(
    temp_dir: P,
    config: AttachmentConfig,
    total_chunks: usize,
) -> AResult<String> {
    let sid = spawn_blocking(move || assemble_file_sync(temp_dir, config, total_chunks)).await??;
    Ok(sid)
}

pub async fn try_mkdirp<P: AsRef<Path>>(path: P) -> EResult {
    let dir = path.as_ref();
    if tokio::fs::metadata(&dir).await.is_err() {
        tokio::fs::create_dir_all(&dir).await?;
    }
    Ok(())
}

pub(super) async fn upload_by_chunks(
    _: HeaderMap,
    state: State<ShareAppState>,
    TypedMultipart(KFileChunkUploadReq {
        filename,
        chunk_no,
        total_chunks,
        chunk,
        last_modified,
        filesize,
        meta_id,
        content_type,
        upload_id,
    }): TypedMultipart<KFileChunkUploadReq>,
) -> AResult<KFileUploadRsp> {
    let mapper = &state.mapper;

    let tmp_dir = asset_tmp_path(&state.config.attachment, &upload_id);

    try_mkdirp(&tmp_dir).await?;

    let chunk_path = tmp_dir.join(chunk_no.to_string());
    let mut file = tokio::fs::File::create(&chunk_path).await?;
    file.write_all(&chunk.contents).await?;

    let mut all_existed = true;

    for p in (0..total_chunks).map(|i| tmp_dir.join(i.to_string())) {
        if !tokio::fs::try_exists(p).await.is_ok_and(|b| b) {
            all_existed = false;
            break;
        }
    }

    let kfile = if all_existed {
        let tid = TID::default();
        let blake3_sum =
            assemble_file(tmp_dir, state.config.attachment.clone(), total_chunks).await?;

        let kfile = KFileMeta {
            tid,
            content_type: content_type.try_into()?,
            filesize,
            sid: blake3_sum.to_string().try_into()?,
            id: meta_id.try_into()?,
            inline: false,
            archor: false,
            filename: filename.try_into()?,
            last_modified: last_modified.try_into()?,
        };

        mapper.insert_kfile(kfile.clone()).await?;
        Some(kfile)
    } else {
        None
    };

    Ok(KFileUploadRsp {
        finished: kfile.is_some(),
        kfile,
    })
}

pub(super) async fn upload_big_file_with_sid(
    headers: HeaderMap,
    state: State<ShareAppState>,
    RestPath(sid): RestPath<String>,
    multipart: Multipart,
) -> KResponse<SharedStr> {
    let filesize = match headers.get("K-filesize") {
        Some(sv) => sv
            .to_str()
            .map(|s| s.parse::<u64>().unwrap_or(0))
            .unwrap_or(0),
        None => 0,
    };
    upload_big_file_with_sid_inner(state, sid, multipart, filesize)
        .await
        .into()
}

#[inline]
async fn upload_big_file_with_sid_inner(
    state: State<ShareAppState>,
    sid: String,
    mut multipart: Multipart,
    filesize: u64,
) -> AResult<SharedStr> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let _ = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        let final_path = state.config.attachment.get_sid_path(sid.as_str());
        let tmp_path = state
            .config
            .attachment
            .get_sid_path((sid.clone() + "_kuploadwhole").as_str());

        try_mkdirp(final_path.parent().context("unable to get parent dir")?).await?;

        if tmp_path.exists() {
            tokio::fs::remove_file(&tmp_path).await?;
        }

        let hash = stream_to_file(field, &tmp_path).await?;
        if hash.as_str() != sid.as_str() {
            warn!("the upload file sid {hash:?} is not same to request {sid}");
        } else {
            let true_size = final_path.metadata()?.len();
            if filesize != true_size {
                bail!(
                    "even the file size is not equal: {} - {}",
                    filesize,
                    final_path.metadata()?.len()
                )
            }
        }
        tokio::fs::rename(tmp_path, final_path).await?;

        return Ok(hash);
    }

    Err(anyhow::anyhow!("Find no file."))
}

async fn stream_to_file<S, E, P: AsRef<Path>>(stream: S, save_file: P) -> AResult<SharedStr>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<axum::BoxError>,
{
    use futures::TryStreamExt;
    async {
        info!("stream to file: {:?}", save_file.as_ref());

        let body_with_io_error = stream.map_err(|err| std::io::Error::other(err));
        let body_reader = tokio_util::io::StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let mut file = BufWriter::new(File::create(&save_file).await?);

        tokio::io::copy(&mut body_reader, &mut file).await?;
        let hash = file_blake3_sum(save_file)?;

        Ok(hash.into())
    }
    .await
}

// https://github.com/tokio-rs/axum/discussions/608
pub(super) async fn download(
    state: State<ShareAppState>,
    axum::extract::Path((meta_otid, filename)): axum::extract::Path<(String, String)>,
) -> impl IntoResponse {
    info!("download sid: {meta_otid}, {filename}");

    async fn inner(
        state: State<ShareAppState>,
        meta_id: &str,
    ) -> AResult<([(HeaderName, String); 2], body::Body)> {
        let kfile = state
            .mapper
            .query_kfile_meta(QueryKFileReq {
                meta_id: meta_id.to_string().try_into()?,
            })
            .await?
            .meta
            .context("unable to find kfile")?;

        let save_filepath = state.config.attachment.get_sid_path(kfile.sid.as_str());

        let file = tokio::fs::File::open(&save_filepath).await?;

        let stream = ReaderStream::new(file);
        let body: body::Body = body::Body::from_stream(stream);

        let headers = [
            (header::CONTENT_TYPE, kfile.content_type.as_str().to_owned()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", &kfile.filename.as_str()),
            ),
        ];
        Ok((headers, body))
    }

    let res = inner(state, &meta_otid).await;

    match res {
        Ok(res) => Ok(res),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unable to download: {:?}, {}", &err.to_string(), err),
        )),
    }
}
