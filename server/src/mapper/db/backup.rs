use chin_sql::PlaceHolderType;
use chin_tools::{AResult, EResult};
use serde::Serialize;

use crate::{
    llmchat::mapper::LLMChatDumpMapper, mapper::{dump::RecordCallbackType, DeserializeMapper, DumpMapper}, model::db::{
        chnot::{ChnotMetadata, ChnotRecord}, kfile::KFile, workspace::{WorkspaceRecord, WorkspaceRelation}
    }
};

use super::{tabledumpsql::TableDumpSqlBuilder, KDb, KDbRow};

impl KDb {
    pub(crate) async fn read_iterator<'a, F1, O>(
        &self,
        sql_builder: TableDumpSqlBuilder<'a>,
        convert_row_to_obj: F1,
        callback: &RecordCallbackType,
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

    async fn dump_and_callback(&self, callback: &RecordCallbackType) -> chin_tools::EResult {
        self.dump_llmchat(callback).await?;
        let s = |name: &'static str| {
            TableDumpSqlBuilder::table(name)
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
            s(WorkspaceRecord::TABLE),
            Self::RowType::to_workspace_record,
            &callback,
        )
        .await?;
        self.read_iterator(
            s(WorkspaceRelation::TABLE),
            Self::RowType::to_workspace_relation,
            &callback,
        )
        .await?;
        self.read_iterator(s(KFile::TABLE), Self::RowType::to_kfile, &callback)
            .await?;

        Ok(())
    }
}
