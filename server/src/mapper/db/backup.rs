use chin_tools::{AResult, EResult};
use serde::Serialize;

use crate::{krate::{chnot::mapper::ChnotDumpMapper, llmchat::mapper::LLMChatDumpMapper}, mapper::DumpMapper, RecordCallbackType};

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
            KDb::Sqlite(_) => todo!(),
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
    
    async fn dump_and_callback(&self, callback: &RecordCallbackType) -> EResult {
        self.dump_llmchat(callback).await?;
        self.dump_chnot(callback).await?;
/*         self.dump_kfile(callback).await?;
        self.dump_ktab(callback).await?;
        self.dump_ktv(callback).await?;
        self.dump_toent(callback).await?; */

        Ok(())
    }

}
