pub mod db;
pub mod dump;
pub mod mappertype;

use chin_tools::{AResult, EResult};
use db::{postgres::PostgresConfig, sqlite::SqliteConfig, KDb};
use dump::RecordCallbackEnum;
use serde::Deserialize;

use crate::model::{
    db::{
        chnot::{ChnotMetadata, ChnotRecord, ChnotTag},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        namespace::{NamespaceRecord, NamespaceRelation},
        resource::{InlineResource, Resource, KV},
    },
    dto::{ctable::*, chnot::*, llmchat::*, resource::*, KReq},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig),
}

pub enum MapperType {
    KDb(KDb),
}

pub trait ChnotMapper {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp>;
    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>>;
    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn chnot_tag_update_single_chnot(
        &self,
        content: &str,
        meta_id: &str,
        namespace: &str,
    ) -> EResult;
    async fn chnot_tag_update_all(&self, namespace: &str) -> EResult;
    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>>;
    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>>;
    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult;
    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult;

    async fn ensure_table_chnot_record(&self) -> EResult;
    async fn ensure_table_chnot_metadata(&self) -> EResult;
    async fn ensure_table_chnot_tag(&self) -> EResult;
}

pub trait ResourceMapper {
    async fn insert_resource(&self, resource: &Resource) -> anyhow::Result<Resource>;
    async fn query_resource_by_id(&self, id: &str) -> anyhow::Result<Resource>;

    ///
    /// Try to insert inline resource.
    ///
    /// Inline resource could keep history if record with archor flag.
    /// If the new version record time is long enough from the old archor,
    /// insert it with archor flag.
    ///
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
    type RowType<'a>;

    async fn dump_and_callback(&self, callback: &RecordCallbackEnum) -> EResult;
}

pub trait ChinTableMapper {
    async fn ctable_overwrite_meta(
        &self,
        req: KReq<CTableOverwriteMetaReq>,
    ) -> AResult<CTableOverwriteMetaRsp>;
    async fn ctable_overwrite_row(
        &self,
        req: KReq<CTableOverwriteRowReq>,
    ) -> AResult<CTableOverwriteRowRsp>;
    async fn ctable_overwrite_cell(
        &self,
        req: KReq<CTableOverwriteCellReq>,
    ) -> AResult<CTableOverwriteCellRsp>;
    async fn ctable_query_row(&self, req: KReq<CTableQueryRowReq>) -> AResult<CTableQueryRowRsp>;
    async fn ctable_query_table_meta(
        &self,
        req: KReq<CTableQueryTableMetaReq>,
    ) -> AResult<CTableQueryTableMetaRsp>;
    async fn ctable_query_table_data(
        &self,
        req: KReq<CTableQueryTableDataReq>,
    ) -> AResult<CTableQueryTableDataRsp>;

    async fn ensure_ctable_tables(&self) -> EResult;
}

pub trait DeserializeMapper {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata>;
    fn to_chnot_record(self) -> AResult<ChnotRecord>;
    fn to_chnot_tag(self) -> AResult<ChnotTag>;

    fn to_llmchat_bot(self) -> AResult<LLMChatBot>;
    fn to_llmchat_template(self) -> AResult<LLMChatTemplate>;
    fn to_llmchat_session(self) -> AResult<LLMChatSession>;
    fn to_llmchat_record(self) -> AResult<LLMChatRecord>;

    fn to_namespace_record(self) -> AResult<NamespaceRecord>;
    fn to_namespace_relation(self) -> AResult<NamespaceRelation>;

    fn to_resource(self) -> AResult<Resource>;
    fn to_inline_resource(self) -> AResult<InlineResource>;

    fn to_kv(self) -> AResult<KV>;
}
