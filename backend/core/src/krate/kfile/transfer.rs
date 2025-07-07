use anyhow::{anyhow, Context};
use axum::{
    body,
    extract::State,
    http::{header, HeaderMap, HeaderName, StatusCode},
    response::IntoResponse,
};
use axum_typed_multipart::TypedMultipart;
use chin_sql::time_type::TID;
use chin_tools::{utils::id_util::generate_uuid, AResult};
use log::info;
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};
use tokio::{io::AsyncWriteExt, task::spawn_blocking};
use tokio_util::io::ReaderStream;

use crate::{
    config::AttachmentConfig,
    krate::kfile::{
        controller::asset_path_by_sid, mapper::KFileMapper, KFileMeta, KFileUploadReq,
        KFileUploadRsp, QueryKFileReq,
    },
    model::omit_tid::OmitTID,
    ShareAppState,
};

pub(crate) fn asset_tmp_path(config: &AttachmentConfig, sid: &str) -> PathBuf {
    std::path::Path::new(&config.base_dir)
        .join("tmp-chunks")
        .join(sid)
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
    let mut bh = blake3::Hasher::new();

    for chunk_number in 0..total_chunks {
        let chunk_path = temp_dir.as_ref().join(chunk_number.to_string());
        let chunk_data = std::fs::read(&chunk_path)?;
        bh.write_all(&chunk_data)?;
        output_file.write_all(&chunk_data)?;
    }

    let sid = bh.finalize().to_string();
    let path = asset_path_by_sid(&config, sid.as_str());
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

pub(super) async fn upload(
    _: HeaderMap,
    state: State<ShareAppState>,
    TypedMultipart(KFileUploadReq {
        filename,
        chunk_no,
        total_chunks,
        chunk,
        last_modified,
        filesize,
        meta_id,
        content_type,
        upload_id,
    }): TypedMultipart<KFileUploadReq>,
) -> AResult<KFileUploadRsp> {
    let mapper = &state.mapper;

    let tmp_dir = asset_tmp_path(&state.config.attachment, &upload_id);

    if tokio::fs::metadata(&tmp_dir).await.is_err() {
        tokio::fs::create_dir_all(&tmp_dir).await?;
    }

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
            omit_tid: OmitTID::never(),
            inline: false,
            archor: false,
            filename: filename.try_into()?,
            last_modified: last_modified.into(),
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

// https://github.com/tokio-rs/axum/discussions/608
pub(crate) async fn download(
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

        let save_filepath = asset_path_by_sid(&state.config.attachment, kfile.sid.as_str());

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
