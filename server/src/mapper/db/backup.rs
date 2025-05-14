use chin_sql::PlaceHolderType;
use chin_tools::{AResult, EResult};
use serde::Serialize;

use crate::{
    mapper::{dump::RecordCallbackEnum, DeserializeMapper, DumpMapper},
    model::db::{
        chnot::{ChnotMetadata, ChnotRecord},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
        namespace::{NamespaceRecord, NamespaceRelation},
        resource::Resource,
    },
};

use super::{tabledumpsql::TableDumpSqlBuilder, KDb, KDbRow};

impl KDb {
    async fn read_iterator<'a, F1, O>(
        &self,
        sql_builder: TableDumpSqlBuilder<'a>,
        convert_row_to_obj: F1,
        callback: &RecordCallbackEnum,
    ) -> EResult
    where
        O: Serialize,
        F1: Fn(KDbRow<'a>) -> AResult<O>,
    {
        match self {
            KDb::Sqlite(sqlite) => todo!(),
            KDb::Postgres(postgres) => {
                postgres
                    .read_iterator(sql_builder, convert_row_to_obj, callback)
                    .await?;
            }
        }

        Ok(())
    }
}

impl DumpMapper for KDb {
    type RowType<'a> = KDbRow<'a>;
    async fn dump_and_callback(&self, callback: &RecordCallbackEnum) -> chin_tools::EResult {
        let s = |name: &'static str| {
            TableDumpSqlBuilder::new(name, None, None, PlaceHolderType::DollarNumber(0))
        };

        self.read_iterator(
            s(ChnotRecord::TABLE),
            Self::RowType::to_chnot_record,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(ChnotMetadata::TABLE),
            Self::RowType::to_chnot_meta,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(NamespaceRecord::TABLE),
            Self::RowType::to_namespace_record,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(NamespaceRelation::TABLE),
            Self::RowType::to_namespace_relation,
            &callback,
        )
        .await?;
        self.read_iterator(s(Resource::TABLE), Self::RowType::to_resource, &callback)
            .await?;
        self.read_iterator(
            s(LLMChatBot::TABLE),
            Self::RowType::to_llmchat_bot,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(LLMChatRecord::TABLE),
            Self::RowType::to_llmchat_record,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(LLMChatSession::TABLE),
            Self::RowType::to_llmchat_session,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(LLMChatTemplate::TABLE),
            Self::RowType::to_llmchat_template,
            &callback,
        )
        .await?;

        Ok(())
    }
}
