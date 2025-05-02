use chin_tools::{
    utils::sort_util,
    wrapper::anyhow::{AResult, EResult},
};

use crate::model::{db::{chnot::ChnotTag, namespace::NamespaceRelation}, dto::InsertInlineResourceRsp};

use super::{
    db::{postgres::Postgres, sqlite::Sqlite},
    dump::RecordCallbackEnum,
    ChnotDeletionRsp, ChnotMapper, ChnotOverwriteReq, ChnotOverwriteRsp, DumpMapper, KVMapper,
    LLMChatMapper, MapperConfig, MapperType, NamespaceMapper, ResourceMapper,
};

use crate::model::{
    db::{namespace::NamespaceRecord, resource::Resource},
    dto::{chnot::*, KReq},
};

impl Into<AResult<MapperType>> for MapperConfig {
    fn into(self) -> AResult<MapperType> {
        match self {
            MapperConfig::Postgres(config) => {
                let pg = Postgres::new(config)?;
                Ok(MapperType::KDb(super::db::KDb::Postgres(pg)))
            }
            MapperConfig::Sqlite(config) => {
                let sqlite = Sqlite::new(config)?;
                Ok(MapperType::KDb(super::db::KDb::Sqlite(sqlite)))
            }
        }
    }
}

impl MapperType {
    pub async fn ensure_tables(&self) -> EResult {
        self.ensure_table_chnot_record().await?;
        self.ensure_table_namespace_record().await?;
        self.ensure_table_namespace_relation().await?;
        self.ensure_table_chnot_metadata().await?;
        self.ensure_table_chnot_tag().await?;
        self.ensure_table_resource().await?;
        self.ensure_table_inline_resource().await?;

        self.ensure_table_llm_chat_bot().await?;
        self.ensure_table_llm_chat_template().await?;
        self.ensure_table_llm_chat_session().await?;
        self.ensure_table_llm_chat_record().await?;

        Ok(())
    }
}

macro_rules! expand_mt_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            MapperType::KDb(db) => db.$method($($arg),*).await,
        }
    };
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        expand_mt_branch!(self.chnot_overwrite(req))
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        expand_mt_branch!(self.chnot_delete(req))
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        expand_mt_branch!(self.chnot_query(req))
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        expand_mt_branch!(self.chnot_update(req))
    }

    async fn ensure_table_chnot_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot_record())
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot_metadata())
    }

    async fn ensure_table_chnot_tag(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot_tag())
    }

    async fn chnot_tag_query(&self, req: KReq<ChnotTagQueryReq>) -> AResult<ChnotTagQueryRsp<ChnotTag>> {
        expand_mt_branch!(self.chnot_tag_query(req))
    }

    async fn chnot_tag_names(&self, req: KReq<ChnotTagQueryReq>) -> AResult<ChnotTagQueryRsp<String>> {
        expand_mt_branch!(self.chnot_tag_names(req))
    }

    async fn chnot_tag_insert(&self, req: crate::model::db::chnot::ChnotTag) -> EResult {
        expand_mt_branch!(self.chnot_tag_insert(req))
    }

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        expand_mt_branch!(self.chnot_tag_delete(chnot_meta_ids))
    }
}

impl ResourceMapper for MapperType {
    async fn insert_resource(&self, resource: &Resource) -> anyhow::Result<Resource> {
        expand_mt_branch!(self.insert_resource(resource))
    }

    async fn query_resource_by_id(&self, id: &str) -> anyhow::Result<Resource> {
        expand_mt_branch!(self.query_resource_by_id(id))
    }

    async fn ensure_table_resource(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_resource())
    }

    async fn insert_inline_resource(
        &self,
        req: &KReq<crate::model::dto::InsertInlineResourceReq>,
    ) -> anyhow::Result<InsertInlineResourceRsp> {
        expand_mt_branch!(self.insert_inline_resource(req))
    }

    async fn query_inline_resource(
        &self,
        req: KReq<crate::model::dto::QueryInlineResourceReq>,
    ) -> anyhow::Result<crate::model::dto::QueryInlineResourceRsp> {
        expand_mt_branch!(self.query_inline_resource(req))
    }

    async fn ensure_table_inline_resource(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_inline_resource())
    }
}

impl NamespaceMapper for MapperType {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>> {
        expand_mt_branch!(self.read_all_namespaces())
    }

    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>> {
        expand_mt_branch!(self.read_all_namespace_relations())
    }

    async fn ensure_table_namespace_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_namespace_record())
    }

    async fn ensure_table_namespace_relation(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_namespace_relation())
    }
}

impl LLMChatMapper for MapperType {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<super::LLMChatOverwriteBotReq>,
    ) -> AResult<super::LLMChatOverwriteBotRsp> {
        expand_mt_branch!(self.llm_chat_overwrite_bot(req))
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<super::LLMChatOverwriteTemplateReq>,
    ) -> AResult<super::LLMChatOverwriteTemplateRsp> {
        expand_mt_branch!(self.llm_chat_overwrite_template(req))
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<super::LLMChatInsertSessionReq>,
    ) -> AResult<super::LLMChatInsertSessionRsp> {
        expand_mt_branch!(self.llm_chat_insert_session(req))
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<super::LLMChatInsertRecordReq>,
    ) -> AResult<super::LLMChatInsertRecordRsp> {
        expand_mt_branch!(self.llm_chat_insert_record(req))
    }

    async fn llm_chat_list_bots(
        &self,
        req: KReq<super::LLMChatListBotReq>,
    ) -> AResult<super::LLMChatListBotRsp> {
        expand_mt_branch!(self.llm_chat_list_bots(req))
    }

    async fn llm_chat_list_templates(
        &self,
        req: KReq<super::LLMChatListTemplateReq>,
    ) -> AResult<super::LLMChatListTemplateRsp> {
        expand_mt_branch!(self.llm_chat_list_templates(req))
    }

    async fn llm_chat_list_sessions(
        &self,
        req: KReq<super::LLMChatListSessionReq>,
    ) -> AResult<super::LLMChatListSessionRsp> {
        expand_mt_branch!(self.llm_chat_list_sessions(req))
    }

    async fn llm_chat_session_detail(
        &self,
        req: KReq<super::LLMChatSessionDetialReq>,
    ) -> AResult<super::LLMChatSessionDetailRsp> {
        let mut raw_result = expand_mt_branch!(self.llm_chat_session_detail(req))?;

        sort_util::sort_by_prev(
            &mut raw_result.records,
            false,
            |r| &r.id,
            |r| &r.pre_record_id,
            |e| &e.insert_time,
        );

        Ok(raw_result)
    }

    async fn llm_chat_update_session(
        &self,
        req: KReq<crate::model::dto::llmchat::LLMChatUpdateSessionReq>,
    ) -> AResult<crate::model::dto::llmchat::LLMChatUpdateSessionRsp> {
        expand_mt_branch!(self.llm_chat_update_session(req))
    }

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<super::LLMChatDeleteBotReq>,
    ) -> AResult<super::LLMChatDeleteBotRsp> {
        expand_mt_branch!(self.llm_chat_delete_bot(req))
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<super::LLMChatDeleteTemplateReq>,
    ) -> AResult<super::LLMChatDeleteTemplateRsp> {
        expand_mt_branch!(self.llm_chat_delete_template(req))
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<super::LLMChatDeleteSessionReq>,
    ) -> AResult<super::LLMChatDeleteSessionRsp> {
        expand_mt_branch!(self.llm_chat_delete_session(req))
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_record())
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_template())
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_session())
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_llm_chat_bot())
    }

    async fn llm_chat_truncate_session(
        &self,
        req: KReq<crate::model::dto::llmchat::LLMChatTruncateSessionReq>,
    ) -> AResult<crate::model::dto::llmchat::LLMChatTruncateSessionRsp> {
        expand_mt_branch!(self.llm_chat_truncate_session(req))
    }
}

impl MapperType {
    pub async fn dump_and_callback(&self, writer: &RecordCallbackEnum) -> EResult {
        expand_mt_branch!(self.dump_and_callback(writer))
    }
}

impl KVMapper for MapperType {
    async fn kv_overwrite(
        &self,
        req: KReq<crate::model::dto::kv::KVOverwriteReq>,
    ) -> AResult<crate::model::dto::kv::KVOverwriteRsp> {
        expand_mt_branch!(self.kv_overwrite(req))
    }

    async fn kv_query(
        &self,
        req: KReq<crate::model::dto::kv::KVQueryReq>,
    ) -> AResult<crate::model::dto::kv::KVQueryRsp> {
        expand_mt_branch!(self.kv_query(req))
    }

    async fn ensure_table_kv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kv())
    }

    async fn kv_delete(&self, req: KReq<super::KVDeleteReq>) -> AResult<super::KVDeleteRsp> {
        expand_mt_branch!(self.kv_delete(req))
    }
}
