use axum::{
    extract::{Query, State},
    http::HeaderMap,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    app::ShareAppState,
    controller::KResponse,
    mapper::ChnotMapper,
    model::v1::dto::{
        req_wrapper, ChnotCommentAddReq, ChnotCommentAddRsp, ChnotCommentDeleteReq,
        ChnotCommentDeleteRsp, ChnotDeletionReq, ChnotDeletionRsp, ChnotInsertionReq,
        ChnotInsertionRsp, ChnotQueryReq, ChnotQueryRsp, ChnotUpdateReq, ChnotUpdateRsp,
    },
};

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot/overwrite", post(chnot_overwrite))
        .route("/api/v1/chnot/update", post(chnot_update))
        .route("/api/v1/chnot/deletion", post(chnot_deletetion))
        .route("/api/v1/chnot/query", get(chnot_query))
        .route("/api/v1/chnot/comment", delete(chnot_comment_deletetion))
        .route("/api/v1/chnot/comment", put(chnot_comment_add))
}

async fn chnot_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotInsertionReq>,
) -> KResponse<ChnotInsertionRsp> {
    state
        .chnot_overwrite(req_wrapper(headers, req))
        .await
        .into()
}

async fn chnot_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotDeletionReq>,
) -> KResponse<ChnotDeletionRsp> {
    state.chnot_deletion(req_wrapper(headers, req)).await.into()
}

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
    state.chnot_query(req_wrapper(headers, req)).await.into()
}

async fn chnot_comment_add(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotCommentAddReq>,
) -> KResponse<ChnotCommentAddRsp> {
    state
        .mapper
        .chnot_comment_add(req_wrapper(headers, req))
        .await
        .into()
}

async fn chnot_comment_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotCommentDeleteReq>,
) -> KResponse<ChnotCommentDeleteRsp> {
    state
        .mapper
        .chnot_comment_delete(req_wrapper(headers, req))
        .await
        .into()
}
