use std::{
    ffi::OsStr,
    io::Write,
    path::{Path, PathBuf},
};

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

use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;
use tracing::info;

use crate::{
    app::ShareAppState,
    config::AttachmentConfig,
    mapper::ResourceMapper,
    model::{
        db::resource::Resource,
        dto::{kreq, read_namespace_from_header, resource::*},
    },
    server::controller::{
        asset::{asset_to_response, ContentEnum},
        KResponse,
    },
};

pub fn asset_path_by_uuid(config: &AttachmentConfig, id: &str) -> PathBuf {
    let filename_parts = split_uuid_to_file_name(&id);

    let save_filepath = std::path::Path::new(&config.base_dir)
        .join(filename_parts.0)
        .join(filename_parts.1)
        .join(filename_parts.2);
    save_filepath
}

pub fn asset_tmp_path(config: &AttachmentConfig, id: &str) -> PathBuf {
    std::path::Path::new(&config.base_dir)
        .join("tmp-chunks")
        .join(id)
}

fn generate_resource_id(filename: &str) -> String {
    let base = uuid::Uuid::new_v4().to_string().replace("-", "");

    match PathBuf::from(filename).extension().and_then(OsStr::to_str) {
        Some(ext) => base + "." + ext,
        None => base,
    }
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
        .write(true)
        .open(filepath.as_ref())
        .await?;
    let mut bh = blake3::Hasher::new();

    for chunk_number in 0..total_chunks {
        let chunk_path = temp_dir.as_ref().join(chunk_number.to_string());
        let chunk_data = tokio::fs::read(&chunk_path).await?;
        bh.write(&chunk_data)?;
        output_file.write_all(&chunk_data).await?;
    }

    // Clean up the temporary chunks
    tokio::fs::remove_dir_all(temp_dir).await?;

    Ok(bh.finalize().to_string())
}

async fn upload(
    headers: HeaderMap,
    state: State<ShareAppState>,
    TypedMultipart(ResourceUploadReq {
        filename,
        chunk_no,
        total_chunks,
        chunk,
        res_id,
        last_modified,
        filesize,
    }): TypedMultipart<ResourceUploadReq>,
) -> AResult<ResourceUploadRsp> {
    let mapper = &state.mapper;

    let tmp_dir = asset_tmp_path(&state.config.attachment, &res_id);

    if !tokio::fs::metadata(&tmp_dir).await.is_ok() {
        tokio::fs::create_dir_all(&tmp_dir).await?;
    }

    let chunk_path = tmp_dir.join(chunk_no.to_string());
    let mut file = tokio::fs::File::create(&chunk_path).await?;
    file.write_all(&chunk.contents).await?;

    let mut all_existed = true;
    for p in (0..total_chunks)
        .into_iter()
        .map(|i| tmp_dir.join(i.to_string()))
    {
        if !tokio::fs::try_exists(p).await.is_ok_and(|b| b) {
            all_existed = false;
            break;
        }
    }

    let resource = if all_existed {
        let id = id_util::generate_uuid();
        let final_filepath = asset_path_by_uuid(&state.config.attachment, &id);
        assemble_file(&tmp_dir, final_filepath, total_chunks).await?;

        let res = mapper
            .insert_resource(&Resource {
                id,
                namespace: read_namespace_from_header(&headers),
                ori_filename: filename,
                content_type: "".to_owned(),
                delete_time: None,
                insert_time: Local::now().into(),
                ori_last_modified: last_modified,
                filesize: filesize,
            })
            .await?;
        Some(res)
    } else {
        None
    };

    Ok(ResourceUploadRsp {
        finished: resource.is_some(),
        resource,
    })
}

// https://github.com/tokio-rs/axum/discussions/608
pub async fn download(
    state: State<ShareAppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("download id: {}", id);

    async fn inner(
        state: State<ShareAppState>,
        id: &str,
    ) -> AResult<([(HeaderName, String); 2], body::Body)> {
        let resource = state.mapper.query_resource_by_id(id).await?;

        let save_filepath = asset_path_by_uuid(&state.config.attachment, &id);

        let file = tokio::fs::File::open(&save_filepath).await?;

        let stream = ReaderStream::new(file);
        let body: body::Body = body::Body::from_stream(stream);

        let headers = [
            (header::CONTENT_TYPE, resource.content_type),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{:?}\"", &resource.ori_filename),
            ),
        ];
        Ok((headers, body))
    }

    let res = inner(state, &id).await;

    match res {
        Ok(res) => return Ok(res),
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unable to download: {:?}, {}", &err.to_string(), err),
            ))
        }
    }
}

async fn query_inline_resource(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<QueryInlineResourceReq>,
) -> KResponse<QueryInlineResourceRsp> {
    state
        .mapper
        .query_inline_resource(kreq(headers, req))
        .await
        .into()
}

async fn insert_inline_resource(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<InsertInlineResourceReq>,
) -> KResponse<InsertInlineResourceRsp> {
    state
        .mapper
        .insert_inline_resource(&kreq(headers, req))
        .await
        .into()
}

async fn query_svg(
    headers: HeaderMap,
    state: State<ShareAppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Response {
    let mut headers = headers.clone();
    headers.append("K-namespace", HeaderValue::from_str("default").unwrap());
    let rsp = query_inline_resource(
        headers,
        state,
        Query(QueryInlineResourceReq {
            id: Some(id.into()),
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
        .and_then(|e| e.res.get(0).cloned())
        .map(|e| ("image/svg+xml", ContentEnum::String(e.content.to_string())));

    asset_to_response(res)
}

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(
            "/api/v1/resource",
            post(|headers, state, mp| async {
                let rsp: KResponse<ResourceUploadRsp> = upload(headers, state, mp).await.into();
                rsp
            })
            .route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route("/api/v1/resource/{id}", get(download))
        .route("/api/v1/inline-resource", put(insert_inline_resource))
        .route("/api/v1/inline-resource", get(query_inline_resource))
        .route("/api/v1/inline-svg/{id}", get(query_svg))
}
