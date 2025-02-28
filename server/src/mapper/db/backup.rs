use anyhow::Context;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use serde::Serialize;
use tokio_postgres::Row;

use crate::{
    mapper::{
        dump::{tabledumpsql::TableDumpSql, DumpWrapper, TableRowCallback, TableRowCallbackEnum},
        DumpMapper, DeserializeMapper,
    },
    to_sql,
};

use super::sql::PlaceHolderType;
use super::Postgres;

impl DumpMapper for Postgres {
    type RowType = Row;

    async fn dump_and_callback(
        &self,
        callback: &TableRowCallbackEnum,
    ) -> chin_tools::wrapper::anyhow::EResult {
        let s = |name: &'static str| {
            TableDumpSql::new(
                name.to_owned(),
                None,
                None,
                PlaceHolderType::DollarNumber(0),
            )
        };

        self.read_iterator(s("chnot_record"), Self::to_chnot_record, &callback)
            .await?;
        self.read_iterator(s("chnot_metadata"), Self::to_chnot_meta, &callback)
            .await?;
        self.read_iterator(s("namespace_record"), Self::to_namespace_record, &callback)
            .await?;
        self.read_iterator(
            s("namespace_relation"),
            Self::to_namespace_relation,
            &callback,
        )
        .await?;
        self.read_iterator(s("resources"), Self::to_resource, &callback)
            .await?;
        self.read_iterator(s("llm_chat_bot"), Self::to_llmchat_bot, &callback)
            .await?;
        self.read_iterator(s("llm_chat_record"), Self::to_llmchat_record, &callback)
            .await?;
        self.read_iterator(s("llm_chat_session"), Self::to_llmchat_session, &callback)
            .await?;
        self.read_iterator(s("llm_chat_template"), Self::to_llmchat_template, &callback)
            .await?;

        Ok(())
    }

    async fn read_iterator<'a, F1, O: Serialize>(
        &self,
        sql_builder: TableDumpSql<'a>,
        convert_row_to_obj: F1,
        callback: &TableRowCallbackEnum,
    ) -> EResult
    where
        F1: Fn(Self::RowType) -> AResult<O>,
    {
        let table_name = sql_builder.table_name.clone();

        let seg = sql_builder.build().context("unable to build dump sql")?;
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
                match convert_row_to_obj(row) {
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
