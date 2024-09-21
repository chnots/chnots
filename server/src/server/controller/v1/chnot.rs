use crate::app::ShareAppState;
use crate::model::dto::{kreq, Chnot};
use crate::{
    mapper::ChnotMapper,
    model::dto::{
        ChnotDeletionReq, ChnotDeletionRsp, ChnotOverwriteRsp, ChnotOverwriteReq, ChnotQueryReq,
        ChnotQueryRsp, ChnotUpdateReq, ChnotUpdateRsp,
    },
    server::controller::KResponse,
};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    routing::{delete, get, post, put},
    Json, Router,
};

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot", put(chnot_overwrite))
        .route("/api/v1/chnot", delete(chnot_deletetion))
        .route("/api/v1/chnot-query", post(chnot_query))
        .route("/api/v1/chnot-update", post(chnot_update))
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
    Json(req): Json<ChnotDeletionReq>,
) -> KResponse<ChnotDeletionRsp> {
    state.mapper.chnot_delete(kreq(headers, req)).await.into()
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
) -> KResponse<ChnotQueryRsp<Vec<Chnot>>> {
    state.chnot_query(kreq(headers, req)).await.into()
}
