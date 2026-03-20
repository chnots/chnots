use crate::controller::KResponse;
use crate::krate::mdwt::mapper::MdwtMapper;
use crate::krate::toent::ToentSearchReq;
use crate::model::dto::kreq;
use crate::{app::ShareAppState, krate::toent::cache::ToentCache};
use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};
use chin_sql::time_type::TID;

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
    let otid = req.mdwt.otid;
    let req = kreq(headers, req);
    let rsp = state.mdwt_commit(req.clone()).await;

    match rsp {
        Ok(rsp) => {
            match ToentCache::refresh_chnots(&state, vec![otid]).await {
                Ok(_) => {}
                Err(err) => return Err(err).into(),
            }
            Ok(rsp).into()
        }
        Err(err) => Err(err).into(),
    }
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

async fn mdwt_tag_refresh(_headers: HeaderMap, state: State<ShareAppState>) -> KResponse<()> {
    state.mdwt_tag_refresh().await.into()
}
