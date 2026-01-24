use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};

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

async fn mind_elixir_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MindElixirCommitReq>,
) -> KResponse<MindElixirCommitRsp> {
    state
        .mapper
        .mind_elixir_commit(kreq(headers, req))
        .await
        .into()
}

async fn mind_elixir_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MindElixirLoadReq>,
) -> KResponse<MindElixirLoadRsp> {
    state
        .mapper
        .mind_elixir_fetch(kreq(headers, req))
        .await
        .into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/excalidraw-commit", post(excalidraw_commit))
        .route("/api/v1/excalidraw-fetch", post(excalidraw_fetch))
        .route("/api/v1/mind-elixir-commit", post(mind_elixir_commit))
        .route("/api/v1/mind-elixir-fetch", post(mind_elixir_fetch))
}
