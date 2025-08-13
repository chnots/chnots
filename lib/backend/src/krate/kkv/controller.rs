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
        .route("/api/v1/kv", get(kv_query))
        .route("/api/v1/kv", delete(kv_delete))
        .route("/api/v1/kv", put(kv_overwrite))
        .route("/api/v1/kkv-query-many", post(kkv_query_many))
}

async fn kv_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KKVOverwriteReq>,
) -> KResponse<KKVOverwriteRsp> {
    state.mapper.kkv_overwrite(kreq(headers, req)).await.into()
}

async fn kv_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<KKVQueryOneReq>,
) -> KResponse<KKVQueryOneRsp> {
    state.mapper.kkv_query(kreq(headers, req)).await.into()
}

async fn kv_delete(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KKVDeleteReq>,
) -> KResponse<KKVDeleteRsp> {
    state.mapper.kkv_delete(kreq(headers, req)).await.into()
}

async fn kkv_query_many(
    state: State<ShareAppState>,
    Json(req): Json<KKVQueryManyReq>,
) -> KResponse<KKVQueryManyRsp> {
    state.mapper.kkv_query_many(req).await.into()
}
