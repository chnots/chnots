use crate::controller::asset::{asset_to_response, ContentEnum};
use axum::{
    body::{self},
    extract::{DefaultBodyLimit, Query, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use axum_typed_multipart::TypedMultipart;
use chin_tools::{
    aanyhow,
    utils::{id_util, path_util::split_uuid_to_file_name},
    AResult,
};
use chrono::Local;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;
use tracing::info;

use crate::{
    app::ShareAppState,
    config::AttachmentConfig,
    model::dto::{kreq, read_kspace_from_header},
    controller::KResponse,
};

use super::{mapper::KFileMapper, *};

pub(crate) fn asset_path_by_uuid(config: &AttachmentConfig, id: &str) -> PathBuf {
    let filename_parts = split_uuid_to_file_name(id);

    let save_filepath = std::path::Path::new(&config.base_dir)
        .join(filename_parts.0)
        .join(filename_parts.1)
        .join(filename_parts.2);
    save_filepath
}

pub(crate) fn asset_tmp_path(config: &AttachmentConfig, id: &str) -> PathBuf {
    std::path::Path::new(&config.base_dir)
        .join("tmp-chunks")
        .join(id)
}

async fn assemble_file<P: AsRef<Path>>(
    temp_dir: &P,
    filepath: P,
    total_chunks: usize,
) -> AResult<String> {
    tokio::fs::create_dir_all(
        filepath
            .as_ref()
            .parent()
            .ok_or(aanyhow!("unable to get parent"))?,
    )
    .await?;
    let mut output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filepath.as_ref())
        .await?;
    let mut bh = blake3::Hasher::new();

    for chunk_number in 0..total_chunks {
        let chunk_path = temp_dir.as_ref().join(chunk_number.to_string());
        let chunk_data = tokio::fs::read(&chunk_path).await?;
        bh.write_all(&chunk_data)?;
        output_file.write_all(&chunk_data).await?;
    }

    // Clean up the temporary chunks
    tokio::fs::remove_dir_all(temp_dir).await?;

    Ok(bh.finalize().to_string())
}

async fn upload(
    headers: HeaderMap,
    state: State<ShareAppState>,
    TypedMultipart(KFileUploadReq {
        filename,
        chunk_no,
        total_chunks,
        chunk,
        res_id,
        last_modified,
        filesize,
    }): TypedMultipart<KFileUploadReq>,
) -> AResult<KFileUploadRsp> {
    let mapper = &state.mapper;

    let tmp_dir = asset_tmp_path(&state.config.attachment, &res_id);

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
        let id = id_util::generate_uuid();
        let final_filepath = asset_path_by_uuid(&state.config.attachment, &id);
        assemble_file(&tmp_dir, final_filepath, total_chunks).await?;

        let res = mapper
            .insert_kfile(&KFile {
                id,
                kspace: read_kspace_from_header(&headers),
                ori_filename: filename,
                content_type: "".to_owned(),
                delete_time: None,
                insert_time: Local::now().into(),
                ori_last_modified: last_modified,
                filesize,
            })
            .await?;
        Some(res)
    } else {
        None
    };

    Ok(KFileUploadRsp {
        finished: kfile.is_some(),
        kfile,
    })
}

pub(crate) async fn kfile_info(
    state: State<ShareAppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> KResponse<QueryKFileRsp> {
    state
        .mapper
        .query_kfile_by_id(&id)
        .await
        .map(|res| QueryKFileRsp { res: Some(res) })
        .into()
}

// https://github.com/tokio-rs/axum/discussions/608
pub(crate) async fn download(
    state: State<ShareAppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("download id: {}", id);

    async fn inner(
        state: State<ShareAppState>,
        id: &str,
    ) -> AResult<([(HeaderName, String); 2], body::Body)> {
        let kfile = state.mapper.query_kfile_by_id(id).await?;

        let save_filepath = asset_path_by_uuid(&state.config.attachment, id);

        let file = tokio::fs::File::open(&save_filepath).await?;

        let stream = ReaderStream::new(file);
        let body: body::Body = body::Body::from_stream(stream);

        let headers = [
            (header::CONTENT_TYPE, kfile.content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{:?}\"", &kfile.ori_filename),
            ),
        ];
        Ok((headers, body))
    }

    let res = inner(state, &id).await;

    match res {
        Ok(res) => Ok(res),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unable to download: {:?}, {}", &err.to_string(), err),
        )),
    }
}

async fn query_inline_kfile(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<QueryInlineKFileReq>,
) -> KResponse<QueryInlineKFileRsp> {
    state
        .mapper
        .query_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn insert_inline_kfile(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<InsertInlineKFileReq>,
) -> KResponse<InsertInlineKFileRsp> {
    state
        .mapper
        .insert_inline_kfile(&kreq(headers, req))
        .await
        .into()
}

async fn query_svg(
    headers: HeaderMap,
    state: State<ShareAppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Response {
    let mut headers = headers.clone();
    headers.append("K-kspace", HeaderValue::from_str("default").unwrap());
    let rsp = query_inline_kfile(
        headers,
        state,
        Query(QueryInlineKFileReq {
            id: Some(id),
            content_type: Some("svg".into()),
            name_like: None,
            rid: None,
            with_del: Some(false),
        }),
    )
    .await
    .0
    .ok();

    let res = rsp
        .and_then(|e| e.res.first().cloned())
        .map(|e| ("image/svg+xml", ContentEnum::String(e.content.to_string())));

    asset_to_response(res)
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(
            "/api/v1/kfile",
            post(|headers, state, mp| async {
                let rsp: KResponse<KFileUploadRsp> = upload(headers, state, mp).await.into();
                rsp
            })
            .route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route("/api/v1/kfile/{id}", get(download))
        .route("/api/v1/kfile-info/{id}", get(kfile_info))
        .route("/api/v1/inline-kfile", put(insert_inline_kfile))
        .route("/api/v1/inline-kfile", get(query_inline_kfile))
        .route("/api/v1/inline-svg/{id}", get(query_svg))
}
