use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::{kreq, read_kspace_from_header};
use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};

use super::mapper::ChnotMapper;
use super::*;

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot-mdwt-commit", post(chnot_mdwt_commit))
        .route("/api/v1/chnot-mdwt-list", post(chnot_mdwt_list))
        .route("/api/v1/chnot-meta-commit", post(chnot_meta_commit))
        .route(
            "/api/v1/chnot-thread-meta-commit",
            post(chnot_thread_meta_commit),
        )
        .route(
            "/api/v1/chnot-thread-meta-fetch}",
            post(chnot_thread_meta_fetch),
        )
        .route(
            "/api/v1/chnot-thread-order-commit",
            post(chnot_thread_order_commit),
        )
        .route("/api/v1/chnot-thread-list", post(chnot_thread_list))
        .route("/api/v1/chnot-tag-list", post(chnot_tag_list))
        .route("/api/v1/chnot-tag-name-list", post(chnot_tag_name_list))
        .route("/api/v1/all-chnot-tag-refresh", post(all_chnot_tag_refresh))
}

async fn chnot_mdwt_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotMdwtCommitReq>,
) -> KResponse<ChnotMdwtCommitRsp> {
    state.chnot_mdwt_commit(kreq(headers, req)).await.into()
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

async fn chnot_mdwt_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtRecordsReq>,
) -> KResponse<MdwtRecordsRsp> {
    state.chnot_mdwt_list(kreq(headers, req)).await.into()
}

async fn chnot_thread_meta_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotThreadMetaFetchCommitReq>,
) -> KResponse<ChnotThreadMetaFetchCommitRsp> {
    state
        .chnot_thread_meta_commit(kreq(headers, req))
        .await
        .into()
}

async fn chnot_thread_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotThreadListReq>,
) -> KResponse<ChnotThreadListRsp> {
    state.chnot_thread_list(kreq(headers, req)).await.into()
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

async fn chnot_tag_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagListReq>,
) -> KResponse<ChnotTagListRsp<ChnotTag>> {
    state.chnot_tag_list(kreq(headers, req)).await.into()
}

async fn chnot_tag_name_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagListReq>,
) -> KResponse<ChnotTagListRsp<String>> {
    state.chnot_tag_name_list(kreq(headers, req)).await.into()
}

async fn all_chnot_tag_refresh(headers: HeaderMap, state: State<ShareAppState>) -> KResponse<()> {
    state
        .all_chnot_tag_refresh(read_kspace_from_header(&headers))
        .await
        .into()
}
