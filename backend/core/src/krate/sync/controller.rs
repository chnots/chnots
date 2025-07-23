use axum::{Json, Router, extract::State, routing::post};

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::sync::dto::{SyncFetchDataReq, SyncFetchDataRsp, SyncShakeReq, SyncShakeRsp},
};

pub const SYNC_SHAKE_PATH: &str = "/api/v1/sync-shake";
pub const SYNC_FETCH_DATA_PATH: &str = "/api/v1/sync-fetch-data";

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(SYNC_SHAKE_PATH, post(sync_shake))
        .route(SYNC_FETCH_DATA_PATH, post(fetch_data))
}

async fn sync_shake(
    state: State<ShareAppState>,
    Json(req): Json<SyncShakeReq>,
) -> KResponse<SyncShakeRsp> {
    state.sync_shake_rx(req).await.into()
}

async fn fetch_data(
    state: State<ShareAppState>,
    Json(req): Json<SyncFetchDataReq>,
) -> KResponse<SyncFetchDataRsp<serde_json::Value>> {
    state.sync_fetch_data_rx(req).await.into()
}
