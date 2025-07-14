use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::{kreq, read_kspace_from_header};
use axum::extract::Query;
use axum::routing::get;
use axum::{
    extract::State,
    http::HeaderMap,
    routing::{delete, post, put},
    Json, Router,
};

use super::mapper::ChnotMapper;
use super::*;

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot-overwrite-record", put(chnot_overwrite_record))
        .route("/api/v1/chnot-overwrite-meta", post(chnot_overwrite_meta))
        .route("/api/v1/chnot", delete(chnot_deletetion))
        .route("/api/v1/chnot-query", post(chnot_query))
        .route("/api/v1/chnot-tag-query", post(chnot_tag_query))
        .route("/api/v1/chnot-tag-names", post(chnot_tag_names))
        .route("/api/v1/chnot-tag-refresh-all", post(chnot_tag_refresh_all))
        .route("/api/v1/chnot-query-kind-rel", get(chnot_query_kind_rel))
}

async fn chnot_overwrite_record(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteRecordReq>,
) -> KResponse<ChnotOverwriteRecordRsp> {
    state
        .chnot_overwrite_record(kreq(headers, req))
        .await
        .into()
}

async fn chnot_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotArchiveReq>,
) -> KResponse<ChnotArchiveRsp> {
    state.mapper.chnot_archive(kreq(headers, req)).await.into()
}

async fn chnot_overwrite_meta(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteMetaReq>,
) -> KResponse<ChnotOverwriteMetaRsp> {
    state.mapper.chnot_overwrite_meta(kreq(headers, req)).await.into()
}

async fn chnot_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotQueryReq>,
) -> KResponse<ChnotQueryRsp<Chnot>> {
    state.mapper.chnot_query(kreq(headers, req)).await.into()
}

async fn chnot_query_kind_rel(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<ChnotKindRelQueryReq>,
) -> KResponse<ChnotKindRelQueryRsp> {
    state
        .mapper
        .chnot_query_kind_rel(kreq(headers, req))
        .await
        .into()
}

async fn chnot_tag_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagQueryReq>,
) -> KResponse<ChnotTagQueryRsp<ChnotTag>> {
    state
        .mapper
        .chnot_tag_query(kreq(headers, req))
        .await
        .into()
}

async fn chnot_tag_names(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagQueryReq>,
) -> KResponse<ChnotTagQueryRsp<String>> {
    state
        .mapper
        .chnot_tag_names(kreq(headers, req))
        .await
        .into()
}

async fn chnot_tag_refresh_all(headers: HeaderMap, state: State<ShareAppState>) -> KResponse<()> {
    state
        .mapper
        .chnot_tag_update_all(read_kspace_from_header(&headers))
        .await
        .into()
}
