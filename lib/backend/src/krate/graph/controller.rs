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

async fn excalidraw_library_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ExcalidrawLibraryCommitReq>,
) -> KResponse<ExcalidrawLibraryCommitRsp> {
    state
        .mapper
        .excalidraw_library_commit(kreq(headers, req))
        .await
        .into()
}

async fn excalidraw_library_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ExcalidrawLibraryFetchReq>,
) -> KResponse<ExcalidrawLibraryFetchRsp> {
    state
        .mapper
        .excalidraw_library_fetch(kreq(headers, req))
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

async fn excalidraw_history_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<GraphHistoryListReq>,
) -> KResponse<GraphHistoryListRsp> {
    state
        .mapper
        .excalidraw_history_list(kreq(headers, req))
        .await
        .into()
}

async fn excalidraw_history_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ExcalidrawHistoryFetchReq>,
) -> KResponse<ExcalidrawHistoryFetchRsp> {
    state
        .mapper
        .excalidraw_history_fetch(kreq(headers, req))
        .await
        .into()
}

async fn excalidraw_history_apply(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ExcalidrawHistoryApplyReq>,
) -> KResponse<ExcalidrawHistoryApplyRsp> {
    state
        .mapper
        .excalidraw_history_apply(kreq(headers, req))
        .await
        .into()
}

async fn mind_elixir_history_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<GraphHistoryListReq>,
) -> KResponse<GraphHistoryListRsp> {
    state
        .mapper
        .mind_elixir_history_list(kreq(headers, req))
        .await
        .into()
}

async fn mind_elixir_history_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MindElixirHistoryFetchReq>,
) -> KResponse<MindElixirHistoryFetchRsp> {
    state
        .mapper
        .mind_elixir_history_fetch(kreq(headers, req))
        .await
        .into()
}

async fn mind_elixir_history_apply(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<MindElixirHistoryApplyReq>,
) -> KResponse<MindElixirHistoryApplyRsp> {
    state
        .mapper
        .mind_elixir_history_apply(kreq(headers, req))
        .await
        .into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/excalidraw-commit", post(excalidraw_commit))
        .route("/api/v1/excalidraw-fetch", post(excalidraw_fetch))
        .route(
            "/api/v1/excalidraw-history-list",
            post(excalidraw_history_list),
        )
        .route(
            "/api/v1/excalidraw-history-fetch",
            post(excalidraw_history_fetch),
        )
        .route(
            "/api/v1/excalidraw-history-apply",
            post(excalidraw_history_apply),
        )
        .route(
            "/api/v1/excalidraw-library-commit",
            post(excalidraw_library_commit),
        )
        .route(
            "/api/v1/excalidraw-library-fetch",
            post(excalidraw_library_fetch),
        )
        .route("/api/v1/mind-elixir-commit", post(mind_elixir_commit))
        .route("/api/v1/mind-elixir-fetch", post(mind_elixir_fetch))
        .route(
            "/api/v1/mind-elixir-history-list",
            post(mind_elixir_history_list),
        )
        .route(
            "/api/v1/mind-elixir-history-fetch",
            post(mind_elixir_history_fetch),
        )
        .route(
            "/api/v1/mind-elixir-history-apply",
            post(mind_elixir_history_apply),
        )
}
