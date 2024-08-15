pub mod backup;
pub mod postgres;

use std::future::Future;

use backup::{BackupTrait, DumpWrapper};
use chin_tools::wrapper::anyhow::{AResult, EResult};
use enum_dispatch::enum_dispatch;
use postgres::{Postgres, PostgresConfig};
use serde::{Deserialize, Serialize};

use crate::model::v1::{db::chnot::Chnot, dto::*};

#[enum_dispatch]
pub enum MapperType {
    Postgres,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
}

impl Into<AResult<MapperType>> for MapperConfig {
    fn into(self) -> AResult<MapperType> {
        match self {
            MapperConfig::Postgres(config) => {
                let pg = Postgres::new(config)?;
                Ok(MapperType::Postgres(pg))
            }
        }
    }
}

#[enum_dispatch(MapperType)]
pub trait TableFounder {
    // Main table.
    async fn _ensure_table_chnots(&self) -> EResult;

    // Comment table.
    async fn _ensure_table_chnot_comments(&self) -> EResult;

    // Toent Definations.
    async fn _ensure_table_toent_defi(&self) -> EResult;

    // Toent Instances.
    async fn _ensure_table_toent_inst(&self) -> EResult;

    async fn _ensure_table_llm_chat(&self) -> EResult;

    // Build all tables
    async fn ensure_tables(&self) -> EResult {
        self._ensure_table_chnots().await?;
        self._ensure_table_chnot_comments().await?;
        self._ensure_table_llm_chat().await?;
        self._ensure_table_toent_defi().await?;
        self._ensure_table_toent_inst().await?;

        Ok(())
    }
}

#[enum_dispatch(MapperType)]
pub trait ChnotMapper {
    async fn chnot_overwrite(
        &self,
        req: ReqWrapper<ChnotInsertionReq>,
    ) -> AResult<ChnotInsertionRsp>;
    async fn chnot_delete(&self, req: ReqWrapper<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: ReqWrapper<ChnotQueryReq>) -> AResult<ChnotQueryRsp>;

    async fn chnot_update(&self, req: ReqWrapper<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn chnot_comment_add(
        &self,
        req: ReqWrapper<ChnotCommentAddReq>,
    ) -> AResult<ChnotCommentAddRsp>;

    async fn chnot_comment_delete(
        &self,
        req: ReqWrapper<ChnotCommentDeleteReq>,
    ) -> AResult<ChnotCommentDeleteRsp>;
}

pub trait LLMChatMapper {
    async fn llm_chat_template_list(req: LLMChatTemplateListReq)
        -> AResult<LLMChatTemplateListRsp>;
    async fn llm_chat_template_overwrite(
        req: LLMChatTemplateOverwriteReq,
    ) -> AResult<LLMChatTemplateOverwriteRsp>;
    async fn llm_chat_template_delete(
        req: LLMChatTemplateDeleteReq,
    ) -> AResult<LLMChatTemplateDeleteRsp>;

    async fn llm_chat_history_list(req: LLMChatHistoryListReq) -> AResult<LLMChatHistoryListRsp>;
    async fn llm_chat_history_detail(
        req: LLMChatHistoryDetailReq,
    ) -> AResult<LLMChatHistoryDetailRsp>;
    async fn llm_chat_history_add(req: LLMChatHistoryAddReq) -> AResult<LLMChatHistoryAddRsp>;

    async fn llm_chat_config_list(req: LLMChatConfigListReq) -> AResult<LLMChatConfigListRsp>;
    async fn llm_chat_config_overwrite(
        req: LLMChatConfigOverwriteReq,
    ) -> AResult<LLMChatConfigOverwriteRsp>;
    async fn llm_chat_config_delete(req: LLMChatConfigDeleteReq)
        -> AResult<LLMChatConfigDeleteRsp>;
}

pub trait Mapper: TableFounder + ChnotMapper {}

impl BackupTrait for MapperType {
    async fn dump_chnots<F, R1>(&self, row_writer: F) -> EResult
    where
        F: Fn(DumpWrapper<Chnot>) -> R1,
        R1: Future<Output = EResult>,
    {
        match self {
            MapperType::Postgres(s) => s.dump_chnots(row_writer).await,
        }
    }
}
