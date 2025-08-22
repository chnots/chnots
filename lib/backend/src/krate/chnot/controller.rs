use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::{kreq, read_kspace_from_header};
use axum::extract::Path;
use axum::routing::get;
use axum::{
    Json, Router,
    extract::State,
    http::HeaderMap,
    routing::{post, put},
};
use chin_sql::time_type::TID;

use super::mapper::ChnotMapper;
use super::*;

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(
            "/api/v1/chnot-overwrite-blocks",
            put(chnot_overwrite_blocks),
        )
        .route("/api/v1/chnot-overwrite-meta", post(chnot_overwrite_meta))
        .route("/api/v1/chnot-meta/{chnot_otid}", get(chnot_detail))
        .route("/api/v1/mdwt-blocks", post(mdwt_blocks))
        .route("/api/v1/chnot-query", post(chnot_query))
        .route("/api/v1/chnot-tag-query", post(chnot_tag_query))
        .route("/api/v1/chnot-tag-names", post(chnot_tag_names))
        .route("/api/v1/chnot-tag-refresh-all", post(chnot_tag_refresh_all))
}

async fn chnot_overwrite_blocks(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteBlockReq>,
) -> KResponse<ChnotOverwriteRecordRsp> {
    state
        .chnot_overwrite_records(kreq(headers, req))
        .await
        .into()
}

async fn mdwt_blocks(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtBlocksReq>,
) -> KResponse<MdwtBlocksRsp> {
    state.mdwt_blocks(kreq(headers, req)).await.into()
}

async fn chnot_overwrite_meta(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteMetaReq>,
) -> KResponse<ChnotOverwriteMetaRsp> {
    state
        .mapper
        .chnot_overwrite_meta(kreq(headers, req))
        .await
        .into()
}

async fn chnot_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotQueryReq>,
) -> KResponse<ChnotQueryRsp<Chnot>> {
    state.mapper.chnot_query(kreq(headers, req)).await.into()
}

async fn chnot_detail(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Path(chnot_otid): Path<TID>,
) -> KResponse<ChnotMetaRsp> {
    state
        .mapper
        .chnot_meta(kreq(
            headers,
            ChnotMetaReq {
                chnot_meta_otid: chnot_otid,
            },
        ))
        .await
        .into()
}

async fn chnot_tag_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotTagQueryReq>,
) -> KResponse<ChnotTagQueryRsp<ChnotTag>> {
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
) -> KResponse<ChnotTagQueryRsp<String>> {
    state
        .mapper
        .chnot_tag_names(kreq(headers, req))
        .await
        .into()
}

async fn chnot_tag_refresh_all(headers: HeaderMap, state: State<ShareAppState>) -> KResponse<()> {
    state
        .mapper
        .chnot_tag_update_all(read_kspace_from_header(&headers))
        .await
        .into()
}
