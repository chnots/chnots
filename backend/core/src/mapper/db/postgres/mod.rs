pub(crate) mod imp;
pub(crate) mod types;
pub(crate) mod wrapper;

use chin_tools::AResult;
use deadpool_postgres::{Client, Pool, PoolError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    user: String,
    pass: String,
    dbname: String,
    host: String,
    port: u16,
}

impl From<PostgresConfig> for deadpool_postgres::Config {
    fn from(val: PostgresConfig) -> Self {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.user = Some(val.user);
        cfg.password = Some(val.pass);
        cfg.dbname = Some(val.dbname);
        cfg.host = Some(val.host);
        cfg.port = Some(val.port);
        cfg
    }
}

pub(crate) struct Postgres {
    pub(crate) pool: Pool,
}

impl Postgres {
    pub(crate) fn new(config: PostgresConfig) -> AResult<Postgres> {
        let pool = Into::<deadpool_postgres::Config>::into(config)
            .create_pool(None, tokio_postgres::NoTls)?;

        Ok(Postgres { pool })
    }

    pub(crate) async fn client(&self) -> Result<Client, PoolError> {
        self.pool.get().await
    }
}
