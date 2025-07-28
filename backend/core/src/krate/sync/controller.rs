use axum::{Json, Router, extract::State, routing::post};
use chin_tools::AResult;

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::{
        chnot::{ChnotKindRel, ChnotMetadata, ChnotRecord, ChnotTag},
        kfile::KFileMeta,
        kkv::KKV,
        kspace::KSpace,
        ktab::{KTabCellDate, KTabCellDecimal, KTabCellText, KTabMeta},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        sync::dto::{
            SyncAllEndpointsReq, SyncAllEndpointsRsp, SyncDataReq, SyncFetchTIDReq,
            SyncFetchTIDRsp, SyncShakeReq, SyncShakeRsp,
        },
    },
    model::KOtidSupport,
    sync_cmds_json_to_st, sync_cmds_st_to_json,
};

use super::dto::SyncDataDto;

pub const SYNC_SHAKE_PATH: &str = "/api/v1/sync-shake";
pub const SYNC_FETCH_TID_PATH: &str = "/api/v1/sync-fetch-tids";
pub const SYNC_DATA_PATH: &str = "/api/v1/sync-data";

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(SYNC_SHAKE_PATH, post(sync_shake))
        .route(SYNC_FETCH_TID_PATH, post(sync_fetch_tids))
        .route(SYNC_DATA_PATH, post(sync_data))
        .route(
            "/api/v1/overwrite-all-sync-endpoints",
            post(overwrite_endpoints),
        )
}

macro_rules! sync_invoke_enum2generic {
    ($invoke:ident, $worker:expr, $eobj:expr) => {{
        use std::marker::PhantomData;
        use $crate::krate::sync::dto::*;
        use $crate::model::otid_table::OtidTableEnum;

        match $eobj.table_type {
            OtidTableEnum::ChnotRecord => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotRecord>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotMetadata => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotMetadata>,
                };
                $worker.$invoke(arg).await
            }
            OtidTableEnum::ChnotKindRel => {
                let arg = OtidWithGer {
                    dto: $eobj.dto,
                    table_type: PhantomData::<crate::krate::chnot::ChnotKindRel>,
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

async fn sync_data_inner(state: State<ShareAppState>, req: SyncDataReq) -> AResult<SyncDataReq> {
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
            SyncDataReq {
                table_type: <$st>::get_otid_enum(),
                dto: SyncDataDto {
                    cmds: sync_cmds_st_to_json!(rsp.cmds),
                    max_tid: req.dto.max_tid,
                    nomore: req.dto.nomore,
                },
            }
        }};
    }
    let result = match req.table_type {
        crate::model::otid_table::OtidTableEnum::ChnotRecord => inner! {ChnotRecord},
        crate::model::otid_table::OtidTableEnum::ChnotMetadata => inner! {ChnotMetadata},
        crate::model::otid_table::OtidTableEnum::ChnotKindRel => inner! {ChnotKindRel},
        crate::model::otid_table::OtidTableEnum::ChnotTag => inner! {ChnotTag},
        crate::model::otid_table::OtidTableEnum::LLMChatBot => inner! {LLMChatBot},
        crate::model::otid_table::OtidTableEnum::LLMChatRecord => inner! {LLMChatRecord},
        crate::model::otid_table::OtidTableEnum::LLMChatTemplate => inner! {LLMChatTemplate},
        crate::model::otid_table::OtidTableEnum::LLMChatSession => inner! {LLMChatSession},
        crate::model::otid_table::OtidTableEnum::KKV => inner! {KKV},
        crate::model::otid_table::OtidTableEnum::KTabMeta => inner! {KTabMeta},
        crate::model::otid_table::OtidTableEnum::KTabCellDate => inner! {KTabCellDate},
        crate::model::otid_table::OtidTableEnum::KTabCellDecimal => inner! {KTabCellDecimal},
        crate::model::otid_table::OtidTableEnum::KTabCellText => inner! {KTabCellText},
        crate::model::otid_table::OtidTableEnum::KFileMeta => inner! {KFileMeta},
        crate::model::otid_table::OtidTableEnum::KSpace => inner! {KSpace},
    };

    Ok(result)
}

async fn overwrite_endpoints(
    state: State<ShareAppState>,
    Json(req): Json<SyncAllEndpointsReq>,
) -> KResponse<SyncAllEndpointsRsp> {
    state.overwrite_endpoints(req.data).await.into()
}

async fn sync_data(
    state: State<ShareAppState>,
    Json(req): Json<SyncDataReq>,
) -> KResponse<SyncDataReq> {
    sync_data_inner(state, req).await.into()
}
