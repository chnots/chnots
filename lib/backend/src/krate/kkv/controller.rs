use super::mapper::*;
use super::*;
use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::kreq;
use axum::extract::Query;
use axum::routing::{get, post};
use axum::{
    Json, Router,
    extract::State,
    http::HeaderMap,
    routing::{delete, put},
};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kkv", get(kv_fetch))
        .route("/api/v1/kkv", delete(kv_archive))
        .route("/api/v1/kkv", put(kv_commit))
        .route("/api/v1/kkv-list", post(kv_list))
}

async fn kv_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KKVCommitReq>,
) -> KResponse<KKVCommitRsp> {
    state.kkv_commit(kreq(headers, req)).await.into()
}

async fn kv_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KKVFetchReq>,
) -> KResponse<KKVFetchRsp> {
    state.kkv_fetch(kreq(headers, req)).await.into()
}

async fn kv_archive(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KKVArchiveReq>,
) -> KResponse<KKVArchiveRsp> {
    state.kkv_archive(kreq(headers, req)).await.into()
}

async fn kv_list(
    state: State<ShareAppState>,
    Json(req): Json<KKVListReq>,
) -> KResponse<KKVListRsp> {
    state.kkv_list(req).await.into()
}
