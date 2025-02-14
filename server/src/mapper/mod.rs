pub mod dump;
pub mod mappertype;
pub mod postgres;

use dump::{tabledumpsql::TableDumpSql, TableRowCallbackEnum};
use chin_tools::wrapper::anyhow::{AResult, EResult};
use postgres::{Postgres, PostgresConfig};
use serde::{Deserialize, Serialize};

use crate::model::{
    db::{
        chnot::{ChnotMetadata, ChnotRecord},
        kv::KV,
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        namespace::{NamespaceRecord, NamespaceRelation},
        resource::Resource,
    },
    dto::{
        chnot::*,
        kv::*,
        llmchat::*,
        InsertInlineResourceReq, InsertInlineResourceRsp, KReq, QueryInlineResourceReq,
        QueryInlineResourceRsp,
    },
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
}

pub enum MapperType {
    Postgres(Postgres),
}

pub trait ChnotMapper {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp>;
    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>>;
    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn ensure_table_chnot_record(&self) -> EResult;
    async fn ensure_table_chnot_metadata(&self) -> EResult;
}

pub trait ResourceMapper {
    async fn insert_resource(&self, resource: &Resource) -> anyhow::Result<Resource>;
    async fn query_resource_by_id(&self, id: &str) -> anyhow::Result<Resource>;
    async fn insert_inline_resource(
        &self,
        req: &KReq<InsertInlineResourceReq>,
    ) -> anyhow::Result<InsertInlineResourceRsp>;
    async fn query_inline_resource(
        &self,
        req: KReq<QueryInlineResourceReq>,
    ) -> anyhow::Result<QueryInlineResourceRsp>;

    async fn ensure_table_resource(&self) -> EResult;
    async fn ensure_table_inline_resource(&self) -> EResult;
}

pub trait NamespaceMapper {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>>;
    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>>;

    async fn ensure_table_namespace_record(&self) -> EResult;
    async fn ensure_table_namespace_relation(&self) -> EResult;
}

pub trait LLMChatMapper {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<LLMChatOverwriteBotReq>,
    ) -> AResult<LLMChatOverwriteBotRsp>;
    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<LLMChatOverwriteTemplateReq>,
    ) -> AResult<LLMChatOverwriteTemplateRsp>;
    async fn llm_chat_insert_session(
        &self,
        req: KReq<LLMChatInsertSessionReq>,
    ) -> AResult<LLMChatInsertSessionRsp>;
    async fn llm_chat_insert_record(
        &self,
        req: KReq<LLMChatInsertRecordReq>,
    ) -> AResult<LLMChatInsertRecordRsp>;

    async fn llm_chat_list_bots(&self, req: KReq<LLMChatListBotReq>) -> AResult<LLMChatListBotRsp>;
    async fn llm_chat_list_templates(
        &self,
        req: KReq<LLMChatListTemplateReq>,
    ) -> AResult<LLMChatListTemplateRsp>;
    async fn llm_chat_list_sessions(
        &self,
        req: KReq<LLMChatListSessionReq>,
    ) -> AResult<LLMChatListSessionRsp>;
    async fn llm_chat_update_session(
        &self,
        req: KReq<LLMChatUpdateSessionReq>,
    ) -> AResult<LLMChatUpdateSessionRsp>;
    async fn llm_chat_session_detail(
        &self,
        req: KReq<LLMChatSessionDetialReq>,
    ) -> AResult<LLMChatSessionDetailRsp>;
    async fn llm_chat_truncate_session(
        &self,
        req: KReq<LLMChatTruncateSessionReq>,
    ) -> AResult<LLMChatTruncateSessionRsp>;

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<LLMChatDeleteBotReq>,
    ) -> AResult<LLMChatDeleteBotRsp>;
    async fn llm_chat_delete_template(
        &self,
        req: KReq<LLMChatDeleteTemplateReq>,
    ) -> AResult<LLMChatDeleteTemplateRsp>;
    async fn llm_chat_delete_session(
        &self,
        req: KReq<LLMChatDeleteSessionReq>,
    ) -> AResult<LLMChatDeleteSessionRsp>;

    async fn ensure_table_llm_chat_bot(&self) -> EResult;
    async fn ensure_table_llm_chat_template(&self) -> EResult;
    async fn ensure_table_llm_chat_session(&self) -> EResult;
    async fn ensure_table_llm_chat_record(&self) -> EResult;
}

pub trait KVMapper {
    async fn kv_overwrite(&self, req: KReq<KVOverwriteReq>) -> AResult<KVOverwriteRsp>;
    async fn kv_query(&self, req: KReq<KVQueryReq>) -> AResult<KVQueryRsp>;
    async fn kv_delete(&self, req: KReq<KVDeleteReq>) -> AResult<KVDeleteRsp>;
    async fn ensure_table_kv(&self) -> EResult;
}

pub trait DumpMapper {
    type RowType;

    async fn dump_and_callback(&self, callback: &TableRowCallbackEnum) -> EResult;

    async fn read_iterator<'a, F1, O: Serialize>(
        &self,
        sql_builder: TableDumpSql<'a>,
        convert_row_to_obj: F1,
        writer: &TableRowCallbackEnum,
    ) -> EResult
    where
        F1: Fn(Self::RowType) -> AResult<O>;
}

pub trait DeserializeMapper {
    type RowType;

    fn to_chnot_meta(row: Self::RowType) -> AResult<ChnotMetadata>;
    fn to_chnot_record(row: Self::RowType) -> AResult<ChnotRecord>;

    fn to_llmchat_bot(row: Self::RowType) -> AResult<LLMChatBot>;
    fn to_llmchat_template(row: Self::RowType) -> AResult<LLMChatTemplate>;
    fn to_llmchat_session(row: Self::RowType) -> AResult<LLMChatSession>;
    fn to_llmchat_record(row: Self::RowType) -> AResult<LLMChatRecord>;

    fn to_namespace_record(row: Self::RowType) -> AResult<NamespaceRecord>;
    fn to_namespace_relation(row: Self::RowType) -> AResult<NamespaceRelation>;

    fn to_resource(row: Self::RowType) -> AResult<Resource>;

    fn to_kv(row: Self::RowType) -> AResult<KV>;
}
