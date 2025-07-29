use super::dto::*;
use super::mapper::*;
use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::kreq;
use axum::extract::Query;
use axum::routing::get;
use axum::routing::post;
use axum::{Json, Router, extract::State, http::HeaderMap, routing::put};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kspace-all", get(kspace_read_all))
        .route("/api/v1/kspace-overwrite", put(kspace_overwrite))
        .route("/api/v1/kspace-deletion", post(kspace_delete))
}

async fn kspace_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KSpaceOverwriteReq>,
) -> KResponse<KSpaceOverwriteRsp> {
    state
        .mapper
        .kspace_overwrite(kreq(headers, req))
        .await
        .into()
}

async fn kspace_read_all(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KSpaceQueryAllReq>,
) -> KResponse<KSpaceQueryAllRsp> {
    state
        .mapper
        .kspace_read_all(kreq(headers, req))
        .await
        .into()
}

async fn kspace_delete(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KSpaceDeletionReq>,
) -> KResponse<KSpaceDeletionRsp> {
    state.mapper.kspace_delete(kreq(headers, req)).await.into()
}
