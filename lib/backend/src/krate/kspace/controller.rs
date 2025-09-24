use super::dto::*;
use super::mapper::*;
use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::kreq;
use axum::extract::Query;
use axum::routing::post;
use axum::{Json, Router, extract::State, http::HeaderMap};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kspace-list", post(kspace_list))
        .route("/api/v1/kspace-commit", post(kspace_commit))
        .route("/api/v1/kspace-archive", post(kspace_archive))
}

async fn kspace_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KSpaceCommitReq>,
) -> KResponse<KSpaceCommitRsp> {
    state.kspace_commit(kreq(headers, req)).await.into()
}

async fn kspace_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KSpaceListReq>,
) -> KResponse<KSpaceListRsp> {
    state.kspace_list(kreq(headers, req)).await.into()
}

async fn kspace_archive(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KSpaceArchiveReq>,
) -> KResponse<KSpaceArchiveRsp> {
    state.kspace_archive(kreq(headers, req)).await.into()
}
