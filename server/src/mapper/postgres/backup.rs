use anyhow::Context;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use serde::Serialize;
use tokio_postgres::Row;
use tracing::info;

use crate::{
    mapper::{
        backup::{tabledumper::TableDumpSql, DumpWrapper, TableRowCallback, TableRowCallbackEnum},
        BackupMapper, DeserializeMapper,
    },
    to_sql,
    util::sql_builder::PlaceHolderType,
};

use super::Postgres;

impl BackupMapper for Postgres {
    type RowType = Row;

    async fn dump_and_backup(
        &self,
        writer: crate::mapper::backup::TableRowCallbackEnum,
    ) -> chin_tools::wrapper::anyhow::EResult {
        let s = |name: &'static str| {
            TableDumpSql::new(
                name.to_owned(),
                None,
                None,
                PlaceHolderType::DollarNumber(0),
            )
        };

        self.read_iterator(s("chnot_record"), Self::to_chnot_record, &writer)
            .await?;
        self.read_iterator(s("chnot_metadata"), Self::to_chnot_meta, &writer)
            .await?;
        self.read_iterator(s("namespace_record"), Self::to_namespace_record, &writer)
            .await?;
        self.read_iterator(
            s("namespace_relation"),
            Self::to_namespace_relation,
            &writer,
        )
        .await?;
        self.read_iterator(s("resources"), Self::to_resource, &writer)
            .await?;
        self.read_iterator(s("llm_chat_bot"), Self::to_llmchat_bot, &writer)
            .await?;
        self.read_iterator(s("llm_chat_record"), Self::to_llmchat_record, &writer)
            .await?;
        self.read_iterator(s("llm_chat_session"), Self::to_llmchat_session, &writer)
            .await?;
        self.read_iterator(s("llm_chat_template"), Self::to_llmchat_template, &writer)
            .await?;

        Ok(())
    }

    async fn read_iterator<'a, F1, O: Serialize>(
        &self,
        sql_builder: TableDumpSql<'a>,
        convert_row_to_obj: F1,
        writer: &TableRowCallbackEnum,
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
                        writer
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
