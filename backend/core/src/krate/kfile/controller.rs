use crate::controller::asset::{asset_to_response, ContentEnum};
use axum::{
    extract::{DefaultBodyLimit, Query, State},
    http::{HeaderMap, HeaderValue},
    response::Response,
    routing::{get, post, put},
    Json, Router,
};

use chin_tools::utils::path_util::split_uuid_to_file_name;
use std::path::PathBuf;

use crate::{
    app::ShareAppState, config::AttachmentConfig, controller::KResponse, model::dto::kreq,
};

use super::{mapper::KFileMapper, *};

pub(crate) fn asset_path_by_sid(config: &AttachmentConfig, sid: &str) -> PathBuf {
    let filename_parts = split_uuid_to_file_name(sid);

    
    std::path::Path::new(&config.base_dir)
        .join(filename_parts.0)
        .join(filename_parts.1)
        .join(filename_parts.2)
}

pub(crate) async fn query_kfile(
    state: State<ShareAppState>,
    Query(req): Query<QueryKFileReq>,
) -> KResponse<QueryKFileMetaRsp> {
    state.mapper.query_kfile_meta(req).await.into()
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
        .insert_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn query_svg(
    headers: HeaderMap,
    state: State<ShareAppState>,
    axum::extract::Path(tid): axum::extract::Path<String>,
) -> Response {
    let mut headers = headers.clone();
    headers.append("K-kspace", HeaderValue::from_str("default").unwrap());
    let rsp = query_inline_kfile(
        headers,
        state,
        Query(QueryInlineKFileReq {
            sid: Some(tid),
            with_omit: Some(false),
            meta_id: None,
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
                let rsp: KResponse<KFileUploadRsp> =
                    transfer::upload(headers, state, mp).await.into();
                rsp
            })
            .route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route(
            "/api/v1/kfile/{meta_tid}/{filename}",
            get(transfer::download),
        )
        .route("/api/v1/kfile-info", get(query_kfile))
        .route("/api/v1/inline-kfile", put(insert_inline_kfile))
        .route("/api/v1/inline-kfile", get(query_inline_kfile))
        .route("/api/v1/inline-svg/{tid}", get(query_svg))
}
