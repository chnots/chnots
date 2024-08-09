use std::ops::Deref;

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use tracing::info;

use crate::{
    app::ShareAppState,
    mapper::ChnotMapper,
    model::v1::dto::{
        req_wrapper, ChnotDeletionReq, ChnotDeletionRsp, ChnotInsertionReq, ChnotInsertionRsp,
        ChnotQueryReq, ChnotQueryRsp, ChnotUpdateReq, ChnotUpdateRsp,
    },
};

use super::KResponse;

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot/overwrite", post(chnot_overwrite))
        .route("/api/v1/chnot/update", post(chnot_update))
        .route("/api/v1/chnot/deletion", post(chnot_deletetion))
        .route("/api/v1/chnot/query", get(chnot_query))
}

async fn chnot_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotInsertionReq>,
) -> KResponse<ChnotInsertionRsp> {
    info!("insert_node: {:?}", req);
    let app = state.0.deref();

    app.chnot_overwrite(req_wrapper(headers, req)).await.into()
}

async fn chnot_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotDeletionReq>,
) -> KResponse<ChnotDeletionRsp> {
    info!("insert_node: {:?}", req);
    let app = state.0.deref();

async fn chnot_update(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotUpdateReq>,
) -> KResponse<ChnotUpdateRsp> {
    state.chnot_update(req_wrapper(headers, req)).await.into()
}

async fn chnot_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<ChnotQueryReq>,
) -> KResponse<ChnotQueryRsp> {
    info!("insert_node: {:?}", req);
    let app = state.0.deref();

    app.chnot_query(req_wrapper(headers, req)).await.into()
}
