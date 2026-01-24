use anyhow::Context;
use axum::{
    Json, Router,
    body::{self, Body},
    extract::{DefaultBodyLimit, Query, State},
    http::{HeaderMap, HeaderName},
    response::IntoResponse,
    routing::{get, post, put},
};
use chin_tools::AResult;

use crate::{app::ShareAppState, controller::KResponse, model::dto::kreq};

use super::{mapper::GraphMapper, *};

async fn excalidraw_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ExcalidrawCommitReq>,
) -> KResponse<ExcalidrawCommitRsp> {
    state
        .mapper
        .excalidraw_commit(kreq(headers, req))
        .await
        .into()
}

async fn excalidraw_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ExcalidrawFetchReq>,
) -> KResponse<ExcalidrawFetchRsp> {
    state
        .mapper
        .excalidraw_fetch(kreq(headers, req))
        .await
        .into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/excalidraw-commit", put(excalidraw_commit))
        .route("/api/v1/excalidraw-fetch", post(excalidraw_fetch))
}
