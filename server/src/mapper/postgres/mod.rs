pub mod chnot;
pub mod llmchat;
pub mod namespace;
pub mod resource;
pub mod backup;


use crate::{model::shared_str::SharedStr, util::sql_builder::SqlValue};
use anyhow::Context;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use deadpool_postgres::{Client, Pool, PoolError};
use postgres_types::{to_sql_checked, FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::model::dto::*;

use super::
    backup::{tabledumpersql::TableDumperSqlBuilder, DbBackupTrait, DumpWrapper, TableDumpWriter, TableDumpWriterEnum}
;

const NO_PARAMS: Vec<&(dyn ToSql + Sync)> = Vec::new();

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    user: String,
    pass: String,
    dbname: String,
    host: String,
    port: u16,
}

impl Into<deadpool_postgres::Config> for PostgresConfig {
    fn into(self) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.user = Some(self.user);
        cfg.password = Some(self.pass);
        cfg.dbname = Some(self.dbname);
        cfg.host = Some(self.host);
        cfg.port = Some(self.port);
        cfg
    }
}

pub struct Postgres {
    pub pool: Pool,
}

impl Postgres {
    pub fn new(config: PostgresConfig) -> AResult<Postgres> {
        let pool = Into::<deadpool_postgres::Config>::into(config)
            .create_pool(None, tokio_postgres::NoTls)?;

        Ok(Postgres { pool })
    }

    async fn client(&self) -> Result<Client, PoolError> {
        self.pool.get().await
    }

    async fn create_table(&self, create_sql: &str) -> EResult {
        self.client()
            .await?
            .execute(create_sql, &[])
            .await
            .map(|_| ())
            .map_err(anyhow::Error::new)?;

        Ok(())
    }
}

impl<'a> Into<&'a (dyn ToSql + Sync + Send)> for &'a SqlValue<'a> {
    fn into(self) -> &'a (dyn ToSql + Sync + Send) {
        match self {
            SqlValue::I8(v) => v,
            SqlValue::I16(v) => v,
            SqlValue::I32(v) => v,
            SqlValue::I64(v) => v,
            SqlValue::Str(v) => v,
            SqlValue::Date(v) => v.as_ref(),
            SqlValue::Bool(v) => v,
            SqlValue::Opt(v) => match v {
                Some(v) => v.as_ref().into(),
                None => &None::<String>,
            },
            SqlValue::SharedStr(shared_str) => shared_str,
        }
    }
}

impl ToSql for SharedStr {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut tokio_util::bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for SharedStr {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <&str as tokio_postgres::types::FromSql>::from_sql(ty, raw)
            .and_then(|s| Ok(SharedStr::new(s)))
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        <&str as tokio_postgres::types::FromSql>::accepts(ty)
    }
}

#[macro_export]
macro_rules! to_sql {
    ($values:expr) => {
        $values
        .iter()
        .map(|e| {
            let v: &(dyn postgres_types::ToSql + Sync + Send) = e.into();
            v as &(dyn postgres_types::ToSql + Sync)
        })
        .collect::<Vec<&(dyn postgres_types::ToSql + Sync)>>()
        .as_slice()
    };
}

impl DbBackupTrait for Postgres {
    type RowType = Row;

    async fn read_iterator<'a, F1, O: Serialize>(
        &self,
        sql_builder: TableDumperSqlBuilder<'a>,
        convert_row_to_obj: F1,
        writer: &TableDumpWriterEnum,
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
                        writer.write_one(DumpWrapper::of(obj, 1, &table_name)).await?;
                    },
                    Err(err) => {
                        tracing::error!("unable to convert {}", err);
                    },
                }
            }

        }

        Ok(())
    }
}
