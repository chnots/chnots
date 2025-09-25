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
    krate::{
        chnot::{ChnotMeta, ChnotThreadMeta},
        kfile::KFileMeta,
        kkv::KKV,
        kspace::KSpace,
        ktab::{KTabCellDate, KTabCellDecimal, KTabCellText, KTabMeta},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        mdwt::{MdwtRecord, MdwtTag},
        sync::{
            dto::{
                SyncDataReqRsp, SyncEndpointCommitReq, SyncEndpointCommitRsp, SyncEndpointListReq,
                SyncEndpointListRsp, SyncEndpointSyncReq, SyncEndpointSyncRsp, SyncShakeReq,
                SyncShakeRsp, SyncTIDListReq, SyncTIDListRsp,
            },
            mapper::SyncMapper,
            po::SyncLogTransientCommit,
        },
        toent::po::MdwtToent,
    },
    model::{KOtidSupport, otid_table::OtidTableEnum},
    sync_cmds_json_to_st, sync_cmds_st_to_json,
};

use super::dto::SyncDataDto;

pub const SYNC_SHAKE_PATH: &str = "/api/v1/b/sync-shake";
pub const SYNC_TID_LIST_PATH: &str = "/api/v1/b/sync-tid-list";
pub const SYNC_DATA_PATH: &str = "/api/v1/b/sync-data";
pub const SYNC_LOG_COMMIT_PATH: &str = "/api/v1/b/sync-insert-sync-log";

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(SYNC_SHAKE_PATH, post(sync_shake))
        .route(SYNC_TID_LIST_PATH, post(sync_tid_list))
        .route(SYNC_DATA_PATH, post(sync_data))
        .route(SYNC_LOG_COMMIT_PATH, post(sync_log_transient_commit))
        .route("/api/v1/sync-endpoint-commit", post(sync_endpoint_commit))
        .route("/api/v1/sync-endpoint-list", post(sync_endpoint_list))
        .route("/api/v1/sync-endpoint-sync", post(sync_endpoint_sync))
}

macro_rules! sync_invoke_enum2generic {
    ($invoke:ident, $worker:expr, $eobj:expr) => {{
        use std::marker::PhantomData;
        use $crate::krate::sync::dto::*;
        use $crate::model::otid_table::OtidTableEnum;

        match $eobj.table_type {
            OtidTableEnum::MdwtRecord => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::mdwt::MdwtRecord>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotThreadMeta => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotThreadMeta>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::MdwtTag => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::mdwt::MdwtTag>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::LLMChatBot => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::llmchat::LLMChatBot>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::LLMChatRecord => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::llmchat::LLMChatRecord>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::LLMChatTemplate => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::llmchat::LLMChatTemplate>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::LLMChatSession => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::llmchat::LLMChatSession>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KKV => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::kkv::KKV>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KTabMeta => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::ktab::KTabMeta>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KTabCellDate => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::ktab::KTabCellDate>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KTabCellDecimal => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::ktab::KTabCellDecimal>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KTabCellText => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::ktab::KTabCellText>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KFileMeta => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::kfile::KFileMeta>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::KSpace => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::kspace::KSpace>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotMeta => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotMeta>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::MdwtToent => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::toent::MdwtToent>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotThreadOrder => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotThreadOrder>,
                };
                $worker.$invoke(arg).await
            }
        }
    }};
}

async fn sync_shake(
    state: State<ShareAppState>,
    Json(req): Json<SyncShakeReq>,
) -> KResponse<SyncShakeRsp> {
    let c = sync_invoke_enum2generic!(sync_shake_rx, state, req);
    c.into()
}

async fn sync_tid_list(
    state: State<ShareAppState>,
    Json(req): Json<SyncTIDListReq>,
) -> KResponse<SyncTIDListRsp> {
    let c = sync_invoke_enum2generic!(sync_tid_list_rx, state, req);
    c.into()
}

async fn sync_data_inner(
    state: State<ShareAppState>,
    req: SyncDataReqRsp,
) -> AResult<SyncDataReqRsp> {
    macro_rules! inner {
        ($st:ty) => {{
            let cmds = req.dto.cmds;
            let cmds = sync_cmds_json_to_st!($st, cmds)?;
            let rsp = state
                .sync_data_rx(SyncDataDto {
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
    let result = match req.table_type {
        OtidTableEnum::MdwtRecord => inner! {MdwtRecord},
        OtidTableEnum::ChnotThreadMeta => inner! {ChnotThreadMeta},
        OtidTableEnum::MdwtTag => inner! {MdwtTag},
        OtidTableEnum::LLMChatBot => inner! {LLMChatBot},
        OtidTableEnum::LLMChatRecord => inner! {LLMChatRecord},
        OtidTableEnum::LLMChatTemplate => inner! {LLMChatTemplate},
        OtidTableEnum::LLMChatSession => inner! {LLMChatSession},
        OtidTableEnum::KKV => inner! {KKV},
        OtidTableEnum::KTabMeta => inner! {KTabMeta},
        OtidTableEnum::KTabCellDate => inner! {KTabCellDate},
        OtidTableEnum::KTabCellDecimal => inner! {KTabCellDecimal},
        OtidTableEnum::KTabCellText => inner! {KTabCellText},
        OtidTableEnum::KFileMeta => inner! {KFileMeta},
        OtidTableEnum::KSpace => inner! {KSpace},
        OtidTableEnum::ChnotMeta => inner! {ChnotMeta},
        OtidTableEnum::MdwtToent => inner! {MdwtToent},
        OtidTableEnum::ChnotThreadOrder => inner! {ChnotThreadMeta},
    };

    info!("{:?}", serde_json::to_string(&result));

    Ok(result)
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

async fn sync_data(
    state: State<ShareAppState>,
    Json(req): Json<SyncDataReqRsp>,
) -> KResponse<SyncDataReqRsp> {
    info!("sync_start: {req:?}");

    let result: Result<super::dto::OtidWithEnum<SyncDataDto<String>>, anyhow::Error> =
        sync_data_inner(state, req).await;
    info!("sync_result: {result:?}");

    result.into()
}

async fn sync_log_transient_commit(
    state: State<ShareAppState>,
    Json(req): Json<SyncLogTransientCommit>,
) -> KResponse<()> {
    state.sync_log_transient_commit(req).await.into()
}
