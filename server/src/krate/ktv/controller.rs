use super::mapper::*;
use super::*;
use crate::app::ShareAppState;
use crate::model::dto::kreq;
use crate::controller::KResponse;
use axum::extract::Query;
use axum::routing::get;
use axum::{
    extract::State,
    http::HeaderMap,
    routing::{delete, put},
    Json, Router,
};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kv", get(kv_query))
        .route("/api/v1/kv", delete(kv_delete))
        .route("/api/v1/kv", put(kv_overwrite))
}

async fn kv_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTVOverwriteReq>,
) -> KResponse<KTVOverwriteRsp> {
    state.mapper.ktv_overwrite(kreq(headers, req)).await.into()
}

async fn kv_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KTVQueryReq>,
) -> KResponse<KTVQueryRsp> {
    state.mapper.ktv_query(kreq(headers, req)).await.into()
}

async fn kv_delete(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTVDeleteReq>,
) -> KResponse<KTVDeleteRsp> {
    state.mapper.ktv_delete(kreq(headers, req)).await.into()
}
