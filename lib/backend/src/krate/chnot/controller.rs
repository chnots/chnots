use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::{PageRsp, kreq};
use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};

use super::mapper::ChnotMapper;
use super::*;

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot-meta-commit", post(chnot_meta_commit))
        .route("/api/v1/chnot-meta-list", post(chnot_meta_list))
        .route(
            "/api/v1/chnot-thread-meta-commit",
            post(chnot_thread_meta_commit),
        )
        .route(
            "/api/v1/chnot-thread-meta-fetch",
            post(chnot_thread_meta_fetch),
        )
        .route(
            "/api/v1/chnot-thread-order-commit",
            post(chnot_thread_order_commit),
        )
        .route("/api/v1/chnot-thread-search", post(chnot_thread_search))
        .route("/api/v1/chnot-single-search", post(chnot_single_search))
}

async fn chnot_thread_order_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotThreadOrderCommitReq>,
) -> KResponse<ChnotThreadOrderCommitRsp> {
    state
        .chnot_overwrite_thread_orders(kreq(headers, req))
        .await
        .into()
}

async fn chnot_meta_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotMetaCommitReq>,
) -> KResponse<ChnotMetaCommitRsp> {
    state.chnot_meta_commit(kreq(headers, req)).await.into()
}

async fn chnot_meta_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotMetaListReq>,
) -> KResponse<ChnotMetaListRsp> {
    state.chnot_meta_list(kreq(headers, req)).await.into()
}

async fn chnot_thread_meta_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotThreadMetaCommitReq>,
) -> KResponse<ChnotThreadMetaCommitRsp> {
    state
        .chnot_thread_meta_commit(kreq(headers, req))
        .await
        .into()
}

async fn chnot_thread_search(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotSearchReq>,
) -> KResponse<PageRsp<ChnotSearchRspThread>> {
    state.chnot_thread_search(kreq(headers, req)).await.into()
}

async fn chnot_single_search(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotSearchReq>,
) -> KResponse<PageRsp<ChnotSearchRspSingle>> {
    state.chnot_single_search(kreq(headers, req)).await.into()
}

async fn chnot_thread_meta_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotThreadMetaFetchReq>,
) -> KResponse<ChnotThreadMetaFetchRsp> {
    state
        .chnot_thread_meta_fetch(kreq(headers, req))
        .await
        .into()
}
