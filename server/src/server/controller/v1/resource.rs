use std::{ffi::OsStr, path::PathBuf};

use axum::{
    body::{self, Bytes},
    extract::{Multipart, Path, Query, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, put},
    Router,
};
use chin_tools::{utils::pathutils::split_uuid_to_file_name, wrapper::anyhow::AResult};
use chrono::Local;
use futures::{Stream, TryStreamExt};

use tokio::{
    fs::File,
    io::{self, BufWriter},
};
use tokio_util::io::ReaderStream;
use tracing::info;

use crate::{
    app::ShareAppState,
    config::AttachmentConfig,
    mapper::ResourceMapper,
    model::{
        db::resource::Resource,
        dto::{
            kreq, read_namespace_from_header, InsertInlineResourceReq, InsertInlineResourceRsp,
            QueryInlineResourceReq, QueryInlineResourceRsp, ResourceUploadRsp,
        },
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

fn generate_resource_id(filename: &str) -> String {
    let base = uuid::Uuid::new_v4().to_string().replace("-", "");

    match PathBuf::from(filename).extension().and_then(OsStr::to_str) {
        Some(ext) => base + "." + ext,
        None => base,
    }
}

async fn upload(
    headers: HeaderMap,
    state: State<ShareAppState>,
    mut multipart: Multipart,
) -> AResult<ResourceUploadRsp> {
    let mut resources = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        let content_type = field.content_type().unwrap().to_string();

        let mapper = &state.mapper;

        let id = generate_resource_id(&filename);

        let save_filepath = asset_path_by_uuid(&state.config.attachment, &id);
        let save_dir = save_filepath.parent().unwrap();

        if !tokio::fs::metadata(&save_dir).await.is_ok() {
            tokio::fs::create_dir_all(&save_dir).await?;
        }

        stream_to_file(field, &save_filepath).await?;

        let res = mapper
            .insert_resource(&Resource {
                id,
                namespace: read_namespace_from_header(&headers),
                ori_filename: filename,
                content_type,
                delete_time: None,
                insert_time: Local::now().into(),
            })
            .await?;
        resources.push(res);
    }

    Ok(ResourceUploadRsp { resources })
}

async fn stream_to_file<S, E>(stream: S, save_file: &PathBuf) -> Result<(), std::io::Error>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<axum::BoxError>,
{
    async {
        let body_with_io_error =
            stream.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        let body_reader = tokio_util::io::StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        let mut file = BufWriter::new(File::create(save_file).await?);

        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
}

// https://github.com/tokio-rs/axum/discussions/608
pub async fn download(state: State<ShareAppState>, Path(id): Path<String>) -> impl IntoResponse {
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
    Query(req): Query<InsertInlineResourceReq>,
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
    Path(id): Path<String>,
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
            put(|headers, state, mp| async {
                let rsp: KResponse<ResourceUploadRsp> = upload(headers, state, mp).await.into();
                rsp
            }),
        )
        .route("/api/v1/resource/{id}", get(download))
        .route("/api/v1/inline-resource", put(insert_inline_resource))
        .route("/api/v1/inline-resource", get(query_inline_resource))
        .route("/api/v1/inline-svg/{id}", get(query_svg))
}
