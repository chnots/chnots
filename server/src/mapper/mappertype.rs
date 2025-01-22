use chin_tools::wrapper::anyhow::{AResult, EResult};

use crate::model::db::namespace::NamespaceRelation;

use super::{
    backup::{BackupTrait, DumpWrapper},
    postgres::Postgres,
    ChnotDeletionRsp, ChnotMapper, ChnotOverwriteReq, ChnotOverwriteRsp, LLMChatMapper,
    MapperConfig, MapperType, NamespaceMapper, ResourceMapper,
};
use std::future::Future;

use crate::model::{
    db::{chnot::ChnotRecord, namespace::NamespaceRecord, resource::Resource},
    dto::{chnot::*, KReq},
};

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

impl MapperType {
    pub async fn ensure_tables(&self) -> EResult {
        self.ensure_table_chnot_record().await?;
        self.ensure_table_namespace_record().await?;
        self.ensure_table_namespace_relation().await?;
        self.ensure_table_chnot_metadata().await?;
        self.ensure_table_resource().await?;
        self.ensure_table_llm_chat_bot().await?;
        self.ensure_table_llm_chat_template().await?;
        self.ensure_table_llm_chat_session().await?;
        self.ensure_table_llm_chat_record().await?;

        Ok(())
    }
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        match self {
            MapperType::Postgres(db) => db.chnot_overwrite(req).await,
        }
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        match self {
            MapperType::Postgres(db) => db.chnot_delete(req).await,
        }
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        match self {
            MapperType::Postgres(db) => db.chnot_query(req).await,
        }
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        match self {
            MapperType::Postgres(db) => db.chnot_update(req).await,
        }
    }

    async fn ensure_table_chnot_record(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_chnot_record().await,
        }
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_chnot_metadata().await,
        }
    }
}

impl ResourceMapper for MapperType {
    async fn insert_resource(&self, resource: &Resource) -> anyhow::Result<Resource> {
        match self {
            MapperType::Postgres(db) => db.insert_resource(resource).await,
        }
    }

    async fn query_resource_by_id(&self, id: &str) -> anyhow::Result<Resource> {
        match self {
            MapperType::Postgres(db) => db.query_resource_by_id(id).await,
        }
    }

    async fn ensure_table_resource(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_resource().await,
        }
    }
}

impl NamespaceMapper for MapperType {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>> {
        match self {
            MapperType::Postgres(db) => db.read_all_namespaces().await,
        }
    }

    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>> {
        match self {
            MapperType::Postgres(db) => db.read_all_namespace_relations().await,
        }
    }

    async fn ensure_table_namespace_record(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_namespace_record().await,
        }
    }

    async fn ensure_table_namespace_relation(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_namespace_relation().await,
        }
    }
}

impl LLMChatMapper for MapperType {
    async fn llm_chat_overwrite_bot(
        &self,
        req: KReq<super::LLMChatOverwriteBotReq>,
    ) -> AResult<super::LLMChatOverwriteBotRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_overwrite_bot(req).await,
        }
    }

    async fn llm_chat_overwrite_template(
        &self,
        req: KReq<super::LLMChatOverwriteTemplateReq>,
    ) -> AResult<super::LLMChatOverwriteTemplateRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_overwrite_template(req).await,
        }
    }

    async fn llm_chat_insert_session(
        &self,
        req: KReq<super::LLMChatInsertSessionReq>,
    ) -> AResult<super::LLMChatInsertSessionRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_insert_session(req).await,
        }
    }

    async fn llm_chat_insert_record(
        &self,
        req: KReq<super::LLMChatInsertRecordReq>,
    ) -> AResult<super::LLMChatInsertRecordRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_insert_record(req).await,
        }
    }

    async fn llm_chat_list_bots(
        &self,
        req: KReq<super::LLMChatListBotReq>,
    ) -> AResult<super::LLMChatListBotRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_list_bots(req).await,
        }
    }

    async fn llm_chat_list_templates(
        &self,
        req: KReq<super::LLMChatListTemplateReq>,
    ) -> AResult<super::LLMChatListTemplateRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_list_templates(req).await,
        }
    }

    async fn llm_chat_list_sessions(
        &self,
        req: KReq<super::LLMChatListSessionReq>,
    ) -> AResult<super::LLMChatListSessionRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_list_sessions(req).await,
        }
    }

    async fn llm_chat_session_detail(
        &self,
        req: KReq<super::LLMChatSessionDetialReq>,
    ) -> AResult<super::LLMChatSessionDetailRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_session_detail(req).await,
        }
    }

    async fn llm_chat_delete_bot(
        &self,
        req: KReq<super::LLMChatDeleteBotReq>,
    ) -> AResult<super::LLMChatDeleteBotRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_delete_bot(req).await,
        }
    }

    async fn llm_chat_delete_template(
        &self,
        req: KReq<super::LLMChatDeleteTemplateReq>,
    ) -> AResult<super::LLMChatDeleteTemplateRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_delete_template(req).await,
        }
    }

    async fn llm_chat_delete_session(
        &self,
        req: KReq<super::LLMChatDeleteSessionReq>,
    ) -> AResult<super::LLMChatDeleteSessionRsp> {
        match self {
            MapperType::Postgres(db) => db.llm_chat_delete_session(req).await,
        }
    }

    async fn ensure_table_llm_chat_record(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_llm_chat_record().await,
        }
    }

    async fn ensure_table_llm_chat_template(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_llm_chat_template().await,
        }
    }

    async fn ensure_table_llm_chat_session(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_llm_chat_session().await,
        }
    }

    async fn ensure_table_llm_chat_bot(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_llm_chat_bot().await,
        }
    }
}

impl BackupTrait for MapperType {
    async fn dump_chnots<F, R1>(&self, _: F) -> EResult
    where
        F: Fn(DumpWrapper<ChnotRecord>) -> R1,
        R1: Future<Output = EResult>,
    {
        match self {
            MapperType::Postgres(_) => {
                tracing::error!("not implement dump logic");
                Ok(())
            }
        }
    }
}
