use std::ops::Deref;

use axum::{extract::State, routing::post, Json, Router};
use tracing::info;

use crate::{
    app::ShareAppState,
    model::dto::{
        ChnotDeletionReq, ChnotDeletionRsp, ChnotInsertionReq, ChnotInsertionRsp, ChnotQueryReq,
        ChnotQueryRsp,
    },
};

use super::KResponse;

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/chnot/overwrite", post(chnot_overwrite))
        .route("/api/v1/chnot/deletion", post(chnot_deletetion))
        .route("/api/v1/chnot/query", post(chnot_query))
}

async fn chnot_overwrite(
    state: State<ShareAppState>,
    Json(chnot): Json<ChnotInsertionReq>,
) -> KResponse<ChnotInsertionRsp> {
    info!("insert_node: {:?}", chnot);
    let app = state.0.deref();

    app.chnot_overwrite(chnot).await.into()
}

async fn chnot_deletetion(
    state: State<ShareAppState>,
    Json(req): Json<ChnotDeletionReq>,
) -> KResponse<ChnotDeletionRsp> {
    info!("insert_node: {:?}", req);
    let app = state.0.deref();

    app.chnot_deletion(req).await.into()
}

async fn chnot_query(
    state: State<ShareAppState>,
    Json(req): Json<ChnotQueryReq>,
) -> KResponse<ChnotQueryRsp> {
    info!("insert_node: {:?}", req);
    let app = state.0.deref();

    app.chnot_query(req).await.into()
}
