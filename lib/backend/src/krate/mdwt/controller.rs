use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::krate::mdwt::mapper::MdwtMapper;
use crate::model::dto::{kreq, read_kspace_from_header};
use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};

use super::*;

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/mdwt-commit", post(mdwt_commit))
        .route("/api/v1/mdwt-list", post(mdwt_list))
        .route("/api/v1/mdwt-tag-list", post(mdwt_tag_list))
        .route("/api/v1/mdwt-tag-name-list", post(mdwt_tag_name_list))
        .route("/api/v1/all-mdwt-tag-refresh", post(mdwt_tag_refresh))
}

async fn mdwt_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtCommitReq>,
) -> KResponse<MdwtCommitRsp> {
    state.mdwt_commit(kreq(headers, req)).await.into()
}

async fn mdwt_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtRecordsReq>,
) -> KResponse<MdwtRecordsRsp> {
    state.mdwt_list(kreq(headers, req)).await.into()
}

async fn mdwt_tag_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtTagListReq>,
) -> KResponse<MdwtTagListRsp<MdwtTag>> {
    state.mdwt_tag_list(kreq(headers, req)).await.into()
}

async fn mdwt_tag_name_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtTagListReq>,
) -> KResponse<MdwtTagListRsp<String>> {
    state.mdwt_tag_name_list(kreq(headers, req)).await.into()
}

async fn mdwt_tag_refresh(headers: HeaderMap, state: State<ShareAppState>) -> KResponse<()> {
    state
        .mdwt_tag_refresh(read_kspace_from_header(&headers))
        .await
        .into()
}
