use crate::app::ShareAppState;
use crate::model::dto::chnot::{Chnot, ChnotTagNamesRsp, ChnotTagQueryReq, ChnotTagQueryRsp};
use crate::model::dto::kreq;
use crate::{
    mapper::ChnotMapper,
    model::dto::chnot::{
        ChnotDeletionReq, ChnotDeletionRsp, ChnotOverwriteReq, ChnotOverwriteRsp, ChnotQueryReq,
        ChnotQueryRsp, ChnotUpdateReq, ChnotUpdateRsp,
    },
    server::controller::KResponse,
};
use axum::{
    extract::State,
    http::HeaderMap,
    routing::{delete, post, put},
    Json, Router,
};

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot", put(chnot_overwrite))
        .route("/api/v1/chnot", delete(chnot_deletetion))
        .route("/api/v1/chnot-query", post(chnot_query))
        .route("/api/v1/chnot-update", post(chnot_update))
        .route("/api/v1/chnot-tag-query", post(chnot_tag_query))
        .route("/api/v1/chnot-tag-names", post(chnot_tag_names))
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

async fn chnot_tag_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagQueryReq>,
) -> KResponse<ChnotTagQueryRsp> {
    state
        .mapper
        .chnot_tag_query(kreq(headers, req))
        .await
        .into()
}

async fn chnot_tag_names(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagQueryReq>,
) -> KResponse<ChnotTagNamesRsp> {
    state
        .mapper
        .chnot_tag_names(kreq(headers, req))
        .await
        .into()
}
