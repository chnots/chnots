use anyhow::Ok;
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::post,
};
use chin_tools::AResult;
use log::info;

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::sync::{dto::*, po::SyncLogTransientCommit},
    model::otid_table::OtidWithEnum,
    otid_enum_generic_invoke, otid_enum_to_generic, sid_enum_to_generic,
};

pub const SYNC_SHAKE_PATH: &str = "/api/v1/b/sync-shake";
pub const SYNC_TID_LIST_PATH: &str = "/api/v1/b/sync-tid-list";
pub const SYNC_DATA_PATH: &str = "/api/v1/b/sync-data";
pub const SYNC_LOG_COMMIT_PATH: &str = "/api/v1/b/sync-insert-sync-log";

pub const SYNC_SID_PO_LIST_PATH: &str = "/api/v1/b/sync-sid-po-fetch";
pub const SYNC_SID_PO_COMMIT_PATH: &str = "/api/v1/b/sync-sid-po-commit";

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(SYNC_SHAKE_PATH, post(sync_shake))
        .route(SYNC_TID_LIST_PATH, post(sync_tid_list))
        .route(SYNC_DATA_PATH, post(sync_data))
        .route(SYNC_LOG_COMMIT_PATH, post(sync_log_transient_commit))
        .route(SYNC_SID_PO_LIST_PATH, post(sync_sid_po_list))
        .route(SYNC_SID_PO_COMMIT_PATH, post(sync_sid_po_commit))
        .route("/api/v1/sync-endpoint-commit", post(sync_endpoint_commit))
        .route("/api/v1/sync-endpoint-list", post(sync_endpoint_list))
        .route("/api/v1/sync-endpoint-sync", post(sync_endpoint_sync))
}

async fn sync_sid_po_list(
    state: State<ShareAppState>,
    Json(req): Json<SyncSidPoListReq<100>>,
) -> KResponse<serde_json::Value> {
    async fn inner(
        req: SyncSidPoListReq<100>,
        state: State<ShareAppState>,
    ) -> AResult<serde_json::Value> {
        macro_rules! handle {
            ($st:ty) => {{
                let list: SyncSidPoGenericDto<$st> =
                    state.sync_sid_po_list_rx::<100, $st>(req.pids).await?;

                Ok(serde_json::to_value(list)?)
            }};
        }
        sid_enum_to_generic!(req.table_type, handle)
    }
    inner(req, state).await.into()
}

async fn sync_sid_po_commit(
    state: State<ShareAppState>,
    Json(req): Json<SyncSidPoCommitReq>,
) -> KResponse<SyncSidPoCommitRsp> {
    async fn inner(
        req: SyncSidPoCommitReq,
        state: State<ShareAppState>,
    ) -> AResult<SyncSidPoCommitRsp> {
        macro_rules! handle {
            ($st:ty) => {{
                let pos: Vec<$st> = req
                    .pos
                    .iter()
                    .map(|e| serde_json::from_str::<$st>(e.as_str()))
                    .collect::<Result<Vec<$st>, serde_json::Error>>()?;
                state
                    .sync_sid_po_commit_rx::<$st>(SyncSidPoGenericDto { pos: pos })
                    .await?;
                Ok(SyncSidPoCommitRsp {})
            }};
        }
        sid_enum_to_generic!(req.table_type, handle)
    }
    inner(req, state).await.into()
}

async fn sync_shake(
    state: State<ShareAppState>,
    Json(req): Json<SyncShakeReq>,
) -> KResponse<SyncShakeRsp> {
    let c = otid_enum_generic_invoke!(sync_shake_rx, state, req);
    c.into()
}

async fn sync_tid_list(
    state: State<ShareAppState>,
    Json(req): Json<SyncOtidTIDListReq>,
) -> KResponse<SyncOtidTIDListRsp> {
    let c = otid_enum_generic_invoke!(sync_otid_tid_list_rx, state, req);
    c.into()
}

async fn sync_endpoint_commit(
    state: State<ShareAppState>,
    Json(req): Json<SyncEndpointCommitReq>,
) -> KResponse<SyncEndpointCommitRsp> {
    state.sync_endpoint_commit(req.data).await.into()
}

async fn sync_endpoint_list(
    state: State<ShareAppState>,
    Query(_): Query<SyncEndpointListReq>,
) -> KResponse<SyncEndpointListRsp> {
    state
        .sync_endpoint_list()
        .await
        .map(|data| SyncEndpointListRsp { data })
        .into()
}

async fn sync_endpoint_sync(
    state: State<ShareAppState>,
    Json(req): Json<SyncEndpointSyncReq>,
) -> KResponse<SyncEndpointSyncRsp> {
    state
        .sync_endpoint_sync(&req.endpoint)
        .await
        .map(|_| SyncEndpointSyncRsp {})
        .into()
}

#[macro_export]
macro_rules! otid_enum_generic_invoke {
    ($invoke:ident, $worker:expr, $obj_with_enum:expr) => {{
        use std::marker::PhantomData;
        use $crate::model::otid_table::OtidWithGeneric;
        use $crate::otid_enum_to_generic;

        macro_rules! to_generic {
            ($st:ty) => {{
                let arg = OtidWithGeneric {
                    dto: $obj_with_enum.dto,
                    table_type: PhantomData::<$st>,
                };
                $worker.$invoke(arg).await
            }};
        }
        otid_enum_to_generic! {$obj_with_enum.table_type, to_generic}
    }};
}

async fn sync_data(
    state: State<ShareAppState>,
    Json(req): Json<SyncDataReqRsp>,
) -> KResponse<SyncDataReqRsp> {
    async fn sync_data_inner(
        state: State<ShareAppState>,
        req: SyncDataReqRsp,
    ) -> AResult<SyncDataReqRsp> {
        macro_rules! sync_data {
            ($st:ty) => {{
                use $crate::model::OtidTableSupport;
                use $crate::sync_cmds_json_to_st;
                use $crate::sync_cmds_st_to_json;

                let cmds = sync_cmds_json_to_st!($st, req.dto.cmds)?;
                let rsp = state
                    .sync_otid_data_rx(SyncDataDto {
                        cmds,
                        max_tid: req.dto.max_tid,
                        nomore: req.dto.nomore,
                    })
                    .await?;
                SyncDataReqRsp {
                    table_type: <$st>::get_otid_enum(),
                    dto: SyncDataDto {
                        cmds: sync_cmds_st_to_json!(rsp.cmds),
                        max_tid: rsp.max_tid,
                        nomore: rsp.nomore,
                    },
                }
            }};
        }

        let result = otid_enum_to_generic!(req.table_type, sync_data);

        Ok(result)
    }

    let result: Result<OtidWithEnum<SyncDataDto<String>>, anyhow::Error> =
        sync_data_inner(state, req).await;
    info!("sync_result: {result:?}");

    result.into()
}

async fn sync_log_transient_commit(
    state: State<ShareAppState>,
    Json(req): Json<SyncLogTransientCommit>,
) -> KResponse<()> {
    state.sync_insert_sync_log_rx(req).await.into()
}
