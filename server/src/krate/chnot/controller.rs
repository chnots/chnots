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
        .route("/api/v1/chnot", put(chnot_overwrite))
        .route("/api/v1/chnot", delete(chnot_deletetion))
        .route("/api/v1/chnot-query", post(chnot_query))
        .route("/api/v1/chnot-update", post(chnot_update))
        .route("/api/v1/chnot-tag-query", post(chnot_tag_query))
        .route("/api/v1/chnot-tag-names", post(chnot_tag_names))
        .route("/api/v1/chnot-tag-refresh-all", post(chnot_tag_refresh_all))
        .route("/api/v1/chnot-query-kind-rel", get(chnot_query_kind_rel))
}

async fn chnot_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteReq>,
) -> KResponse<ChnotOverwriteRsp> {
    state
        .mapper
        .chnot_overwrite(kreq(headers, req))
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

async fn chnot_update(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotUpdateReq>,
) -> KResponse<ChnotUpdateRsp> {
    state.mapper.chnot_update(kreq(headers, req)).await.into()
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
        .chnot_tag_update_all(read_kspace_from_header(&headers).try_into().unwrap())
        .await
        .into()
}
