use chin_tools::wrapper::anyhow::{AResult, EResult};
use serde::Serialize;

use crate::{
    model::db::{
        chnot::ChnotMetadata,
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        namespace::NamespaceRelation,
    },
    util::{sort_util, sql_builder::PlaceHolderType},
};

use super::{
    backup::{
        tabledumpersql::TableDumperSqlBuilder, DbBackupTrait, DumpWrapper, TableDumpWriter,
        TableDumpWriterEnum, TableDumpWriterType,
    },
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
        let mut raw_result = match self {
            MapperType::Postgres(db) => db.llm_chat_session_detail(req).await,
        }?;

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
        match self {
            MapperType::Postgres(db) => db.llm_chat_update_session(req).await,
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
    
    async fn llm_chat_truncate_session(&self, req: KReq<crate::model::dto::llmchat::LLMChatTruncateSessionReq>) -> AResult<crate::model::dto::llmchat::LLMChatTruncateSessionRsp> {
        todo!()
    }
}

impl MapperType {
    pub async fn dump_and_backup(&self, writer: TableDumpWriterEnum) -> EResult {
        match self {
            MapperType::Postgres(db) => {
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "chnot_record".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let chnot = ChnotRecord {
                            id: row.try_get("id")?,
                            meta_id: row.try_get("meta_id")?,
                            content: row.try_get("content")?,
                            omit_time: row.try_get("omit_time")?,
                            insert_time: row.try_get("insert_time")?,
                        };
                        Ok(chnot)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "chnot_metadata".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let chnot = ChnotMetadata {
                            id: row.try_get("id")?,
                            namespace: row.try_get("namespace")?,
                            kind: row.try_get("kind")?,
                            pin_time: row.try_get("pin_time")?,
                            delete_time: row.try_get("delete_time")?,
                            update_time: row.try_get("update_time")?,
                            insert_time: row.try_get("insert_time")?,
                        };
                        Ok(chnot)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "namespace_record".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = NamespaceRecord {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            name: row.try_get("name")?,
                            delete_time: row.try_get("delete_time")?,
                            update_time: row.try_get("update_time")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "namespace_relation".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = NamespaceRelation {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            delete_time: row.try_get("delete_time")?,
                            update_time: row.try_get("update_time")?,
                            sub_id: row.try_get("sub_id")?,
                            parent_id: row.try_get("parent_id")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "resources".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = Resource {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            delete_time: row.try_get("delete_time")?,
                            namespace: row.try_get("namespace")?,
                            ori_filename: row.try_get("ori_filename")?,
                            content_type: row.try_get("content_type")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "llm_chat_bot".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = LLMChatBot {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            delete_time: row.try_get("delete_time")?,
                            name: row.try_get("name")?,
                            body: row.try_get("body")?,
                            update_time: row.try_get("update_time")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "llm_chat_record".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = LLMChatRecord {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            session_id: row.try_get("session_id")?,
                            pre_record_id: row.try_get("pre_record_id")?,
                            content: row.try_get("content")?,
                            role: row.try_get("role")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "llm_chat_session".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = LLMChatSession {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            bot_id: row.try_get("bot_id")?,
                            template_id: row.try_get("template_id")?,
                            title: row.try_get("title")?,
                            namespace: row.try_get("namespace")?,
                            delete_time: row.try_get("delete_time")?,
                            update_time: row.try_get("update_time")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
                db.read_iterator(
                    TableDumperSqlBuilder::new(
                        "llm_chat_template".to_owned(),
                        None,
                        None,
                        PlaceHolderType::DollarNumber(0),
                    ),
                    |row| {
                        let obj = LLMChatTemplate {
                            id: row.try_get("id")?,
                            insert_time: row.try_get("insert_time")?,
                            delete_time: row.try_get("delete_time")?,
                            update_time: row.try_get("update_time")?,
                            name: row.try_get("name")?,
                            prompt: row.try_get("prompt")?,
                            icon_name: row.try_get("icon_name")?,
                        };
                        Ok(obj)
                    },
                    &writer,
                )
                .await?;
            }
        }
        Ok(())
    }
}
