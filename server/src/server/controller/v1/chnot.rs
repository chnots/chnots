use crate::app::ShareAppState;
use crate::model::v1::dto::{ChnotRing, NestedChnot};
use crate::{
    mapper::ChnotMapper,
    model::v1::dto::{
        kreq, ChnotCommentAddReq, ChnotCommentAddRsp, ChnotCommentDeleteReq, ChnotCommentDeleteRsp,
        ChnotDeletionReq, ChnotDeletionRsp, ChnotInsertionReq, ChnotInsertionRsp, ChnotQueryReq,
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
    Query(req): Query<ChnotQueryReq>,
) -> KResponse<ChnotQueryRsp<Vec<ChnotRing>>> {
    state.chnot_query(kreq(headers, req)).await.into()
}
