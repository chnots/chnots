use anyhow::Context;
use chin_tools::{AResult, EResult};
use serde::Serialize;

use crate::{
    mapper::{
        db::{tabledumpsql::TableDumpSqlBuilder, KDbRow},
        dump::{DumpWrapper, RecordCallbackEnum, RowCallback},
    },
    to_sql,
};

use super::Postgres;

impl Postgres {
    pub async fn read_iterator<'a, F1, O>(
        &self,
        sql_builder: TableDumpSqlBuilder<'a>,
        convert_row_to_obj: F1,
        callback: &RecordCallbackEnum,
    ) -> EResult
    where
        O: Serialize,
        F1: Fn(KDbRow<'a>) -> AResult<O>,
    {
        let table_name = sql_builder.table_name;

        let seg = sql_builder.build(chin_sql::DbType::Postgres).context("unable to build dump sql")?;
        let mut client = self.client().await?;
        let stmt = client.transaction().await?;
        let portal = stmt.bind(&seg.seg, &to_sql!(seg.values)).await?;
        loop {
            // poll batch_size rows from portal and send it to embedding thread via channel
            let rows = stmt.query_portal(&portal, 10 as i32).await?;

            if rows.len() == 0 {
                break;
            }

            for row in rows {
                match convert_row_to_obj(KDbRow::Postgres(row)) {
                    Ok(obj) => {
                        callback
                            .callback(DumpWrapper::of(obj, 1, &table_name))
                            .await?;
                    }
                    Err(err) => {
                        tracing::error!("{} -- unable to convert {}", table_name, err);
                    }
                }
            }
        }

        Ok(())
    }
}
