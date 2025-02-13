use crate::app::ShareAppState;
use crate::mapper::KVMapper;
use crate::model::dto::kreq;
use crate::model::dto::kv::{KVDeleteReq, KVDeleteRsp, KVOverwriteReq, KVOverwriteRsp, KVQueryReq, KVQueryRsp};
use crate::server::controller::KResponse;
use axum::extract::Path;
use axum::routing::get;
use axum::{
    extract::State,
    http::HeaderMap,
    routing::{delete, put},
    Json, Router,
};

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kv", put(kv_overwrite))
        .route("/api/v1/kv", get(kv_query))
        .route("/api/v1/kv", delete(kv_delete)) 
}

async fn kv_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KVOverwriteReq>,
) -> KResponse<KVOverwriteRsp> {
    state.mapper.kv_overwrite(kreq(headers, req)).await.into()
}

async fn kv_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Path(req): Path<KVQueryReq>,
) -> KResponse<KVQueryRsp> {
    state.mapper.kv_query(kreq(headers, req)).await.into()
}

async fn kv_delete(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KVDeleteReq>,
) -> KResponse<KVDeleteRsp> {
    state.mapper.kv_delete(kreq(headers, req)).await.into()
}
