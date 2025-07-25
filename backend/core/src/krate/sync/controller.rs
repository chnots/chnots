use axum::{Json, Router, extract::State, routing::post};

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::sync::dto::{SyncFetchTIDReq, SyncFetchTIDRsp, SyncShakeReq, SyncShakeRsp},
};

pub const SYNC_SHAKE_PATH: &str = "/api/v1/sync-shake";
pub const SYNC_FETCH_TID_PATH: &str = "/api/v1/sync-fetch-tids";
pub const SYNC_SYNC_DATA_PATH: &str = "/api/v1/sync-data";

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(SYNC_SHAKE_PATH, post(sync_shake))
        .route(SYNC_FETCH_TID_PATH, post(sync_fetch_tids))
}

async fn sync_shake(
    state: State<ShareAppState>,
    Json(req): Json<SyncShakeReq>,
) -> KResponse<SyncShakeRsp> {
    state.sync_shake_rx(req).await.into()
}

async fn sync_fetch_tids(
    state: State<ShareAppState>,
    Json(req): Json<SyncFetchTIDReq>,
) -> KResponse<SyncFetchTIDRsp> {
    state.sync_fetch_tids_rx(req).await.into()
}
