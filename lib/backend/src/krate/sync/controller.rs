use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use chin_tools::AResult;
use log::info;

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::{
        chnot::{ChnotMeta, ChnotTag, ChnotThreadMetaFetch, ChnotToent, MdwtRecord},
        kfile::KFileMeta,
        kkv::KKV,
        kspace::KSpace,
        ktab::{KTabCellDate, KTabCellDecimal, KTabCellText, KTabMeta},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        sync::{
            dto::{
                GetSyncAllEndpointsReq, GetSyncAllEndpointsRsp, SyncAllEndpointsReq,
                SyncAllEndpointsRsp, SyncDataReqRsp, SyncFetchTIDReq, SyncFetchTIDRsp,
                SyncShakeReq, SyncShakeRsp, SyncToEndpointReq, SyncToEndpointRsp,
            },
            mapper::SyncMapper,
            po::SyncLogTransient,
        },
    },
    model::{KOtidSupport, otid_table::OtidTableEnum},
    sync_cmds_json_to_st, sync_cmds_st_to_json,
};

use super::dto::SyncDataDto;

pub const SYNC_SHAKE_PATH: &str = "/api/v1/b/sync-shake";
pub const SYNC_FETCH_TID_PATH: &str = "/api/v1/b/sync-fetch-tids";
pub const SYNC_DATA_PATH: &str = "/api/v1/b/sync-data";
pub const SYNC_INSERT_SYNC_LOG: &str = "/api/v1/b/sync-insert-sync-log";

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(SYNC_SHAKE_PATH, post(sync_shake))
        .route(SYNC_FETCH_TID_PATH, post(sync_fetch_tids))
        .route(SYNC_DATA_PATH, post(sync_data))
        .route(
            "/api/v1/overwrite-all-sync-endpoints",
            post(overwrite_endpoints),
        )
        .route("/api/v1/get-all-sync-endpoints", get(fetch_endpoints))
        .route("/api/v1/sync-end-endpoint", post(sync_to_endpoint))
        .route(SYNC_INSERT_SYNC_LOG, post(sync_insert_sync_log))
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
                    table_type: PhantomData::<crate::krate::chnot::MdwtRecord>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotThreadMetaFetch => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotThreadMetaFetch>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotTag => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotTag>,
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
            OtidTableEnum::ChnotToent => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotToent>,
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

async fn sync_fetch_tids(
    state: State<ShareAppState>,
    Json(req): Json<SyncFetchTIDReq>,
) -> KResponse<SyncFetchTIDRsp> {
    let c = sync_invoke_enum2generic!(sync_fetch_tids_rx, state, req);
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
        OtidTableEnum::ChnotThreadMetaFetch => inner! {ChnotThreadMetaFetch},
        OtidTableEnum::ChnotTag => inner! {ChnotTag},
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
        OtidTableEnum::ChnotToent => inner! {ChnotToent},
        OtidTableEnum::ChnotThreadOrder => inner! {ChnotThreadMetaFetch},
    };

    info!("{:?}", serde_json::to_string(&result));

    Ok(result)
}

async fn overwrite_endpoints(
    state: State<ShareAppState>,
    Json(req): Json<SyncAllEndpointsReq>,
) -> KResponse<SyncAllEndpointsRsp> {
    state.overwrite_endpoints(req.data).await.into()
}

async fn fetch_endpoints(
    state: State<ShareAppState>,
    Query(_): Query<GetSyncAllEndpointsReq>,
) -> KResponse<GetSyncAllEndpointsRsp> {
    state
        .get_all_endpoints()
        .await
        .map(|data| GetSyncAllEndpointsRsp { data })
        .into()
}

async fn sync_to_endpoint(
    state: State<ShareAppState>,
    Json(req): Json<SyncToEndpointReq>,
) -> KResponse<SyncToEndpointRsp> {
    state
        .sync_to_endpoint(&req.endpoint)
        .await
        .map(|_| SyncToEndpointRsp {})
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

async fn sync_insert_sync_log(
    state: State<ShareAppState>,
    Json(req): Json<SyncLogTransient>,
) -> KResponse<()> {
    state.sync_insert_sync_log(req).await.into()
}
