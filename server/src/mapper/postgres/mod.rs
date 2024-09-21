pub mod chnot;
pub mod resource;
pub mod namespace;

use crate::
    util::sql_builder::SqlValue
;
use chin_tools::
    wrapper::anyhow::{AResult, EResult}
;
use deadpool_postgres::{Client, Pool, PoolError};
use postgres_types::ToSql;
use serde::Deserialize;


use crate::model::dto::*;

use super::ChnotMapper;

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
        }
    }
}
