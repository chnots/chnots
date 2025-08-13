use super::dto::*;
use super::mapper::*;
use crate::app::ShareAppState;
use crate::controller::KResponse;
use crate::model::dto::kreq;
use axum::routing::post;
use axum::{Json, Router, extract::State, http::HeaderMap, routing::put};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/ktab-meta-read", post(ktab_meta_read))
        .route("/api/v1/ktab-meta-overwrite", put(ktab_meta_overwrite))
        .route("/api/v1/ktab-cells-read", post(ktab_cells_read))
        .route("/api/v1/ktab-cells-overwrite", put(ktab_cells_overwrite))
}

async fn ktab_meta_read(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabMetaQueryReq>,
) -> KResponse<KTabMetaQueryRsp> {
    state
        .mapper
        .ktab_query_table_meta(kreq(headers, req))
        .await
        .into()
}

async fn ktab_meta_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabMetaOverwriteReq>,
) -> KResponse<KTabMetaOverwriteRsp> {
    state
        .mapper
        .ktab_overwrite_meta(kreq(headers, req))
        .await
        .into()
}

async fn ktab_cells_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabCellsOverwriteReq>,
) -> KResponse<KTabCellsOverwriteRsp> {
    state
        .mapper
        .ktab_overwrite_cells(kreq(headers, req))
        .await
        .into()
}

async fn ktab_cells_read(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KTabRowsQueryReq>,
) -> KResponse<KTabRowsQueryRsp> {
    state
        .mapper
        .ktab_query_table_data(kreq(headers, req))
        .await
        .into()
}
