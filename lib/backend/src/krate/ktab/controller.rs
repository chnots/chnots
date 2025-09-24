use super::dto::*;
use super::mapper::*;
use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::kreq;
use axum::routing::post;
use axum::{Json, Router, extract::State, http::HeaderMap};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/ktab-meta-fetch", post(ktab_meta_fetch))
        .route("/api/v1/ktab-meta-commit", post(ktab_meta_commit))
        .route("/api/v1/ktab-cell-list", post(ktab_cell_list))
        .route("/api/v1/ktab-cell-commit", post(ktab_cell_commit))
}

async fn ktab_meta_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabMetaFetchReq>,
) -> KResponse<KTabMetaFetchRsp> {
    state.ktab_meta_fetch(kreq(headers, req)).await.into()
}

async fn ktab_meta_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabMetaCommitReq>,
) -> KResponse<KTabMetaCommitRsp> {
    state.ktab_meta_commit(kreq(headers, req)).await.into()
}

async fn ktab_cell_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabCellCommitReq>,
) -> KResponse<KTabCellCommitRsp> {
    state
        .mapper
        .ktab_cell_commit(kreq(headers, req))
        .await
        .into()
}

async fn ktab_cell_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabCellListReq>,
) -> KResponse<KTabCellListRsp> {
    state.mapper.ktab_cell_list(kreq(headers, req)).await.into()
}
