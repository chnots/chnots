use super::dto::*;
use super::mapper::*;
use crate::app::ShareAppState;
use crate::model::dto::kreq;
use crate::controller::KResponse;
use axum::extract::Query;
use axum::routing::get;
use axum::{
    extract::State,
    http::HeaderMap,
    routing::put,
    Json, Router,
};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kspace-all", get(kspace_read_all))
        .route("/api/v1/kspace", put(kspace_overwrite))
}

async fn kspace_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KSpaceOverwriteReq>,
) -> KResponse<KSpaceOverwriteRsp> {
    state.mapper.kspace_overwrite(kreq(headers, req)).await.into()
}

async fn kspace_read_all(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KSpaceQueryAllReq>,
) -> KResponse<KSpaceQueryAllRsp> {
    state.mapper.kspace_read_all(kreq(headers, req)).await.into()
}