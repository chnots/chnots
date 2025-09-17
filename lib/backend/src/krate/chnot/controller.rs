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
        .route("/api/v1/chnot-overwrite-mdwts", put(chnot_overwrite_mdwts))
        .route("/api/v1/chnot-overwrite-metas", put(chnot_overwrite_metas))
        .route(
            "/api/v1/chnot-overwrite-thread-orders",
            put(chnot_overwrite_thread_orders),
        )
        .route(
            "/api/v1/chnot-thread-overwrite-meta",
            post(chnot_thread_overwrite_meta),
        )
        .route(
            "/api/v1/chnot-thread-meta/{chnot_otid}",
            get(chnot_thread_meta),
        )
        .route("/api/v1/mdwt-records", post(mdwt_records))
        .route("/api/v1/chnot-thread-query", post(chnot_thread_query))
        .route(
            "/api/v1/chnot-thread-tag-query",
            post(chnot_thread_tag_query),
        )
        .route(
            "/api/v1/chnot-thread-tag-names",
            post(chnot_thread_tag_names),
        )
        .route(
            "/api/v1/chnot-thread-tag-refresh-all",
            post(chnot_thread_tag_refresh_all),
        )
}

async fn chnot_overwrite_mdwts(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteMdwtReq>,
) -> KResponse<ChnotOverwriteMdwtRsp> {
    state.chnot_overwrite_mdwt(kreq(headers, req)).await.into()
}

async fn chnot_overwrite_thread_orders(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteThreadOrderReq>,
) -> KResponse<ChnotOverwriteThreadOrderRsp> {
    state
        .chnot_overwrite_thread_orders(kreq(headers, req))
        .await
        .into()
}

async fn chnot_overwrite_metas(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteMetaReq>,
) -> KResponse<ChnotOverwriteMetaRsp> {
    state.chnot_overwrite_metas(kreq(headers, req)).await.into()
}

async fn mdwt_records(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MdwtRecordsReq>,
) -> KResponse<MdwtRecordsRsp> {
    state.mdwt_blocks(kreq(headers, req)).await.into()
}

async fn chnot_thread_overwrite_meta(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotOverwriteThreadMetaReq>,
) -> KResponse<ChnotOverwriteThreadMetaRsp> {
    state
        .mapper
        .chnot_overwrite_thread_meta(kreq(headers, req))
        .await
        .into()
}

async fn chnot_thread_query(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ChnotThreadQueryReq>,
) -> KResponse<ChnotThreadQueryRsp> {
    state
        .mapper
        .chnot_thread_query(kreq(headers, req))
        .await
        .into()
}

async fn chnot_thread_meta(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Path(chnot_otid): Path<TID>,
) -> KResponse<ChnotThreadMetaRsp> {
    state
        .mapper
        .chnot_thread_meta(kreq(
            headers,
            ChnotThreadMetaReq {
                thread_otid: chnot_otid,
            },
        ))
        .await
        .into()
}

async fn chnot_thread_tag_query(
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

async fn chnot_thread_tag_names(
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

async fn chnot_thread_tag_refresh_all(
    headers: HeaderMap,
    state: State<ShareAppState>,
) -> KResponse<()> {
    state
        .mapper
        .chnot_tag_update_all(read_kspace_from_header(&headers))
        .await
        .into()
}
