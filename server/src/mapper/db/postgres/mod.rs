pub(crate) mod imp;
pub(crate) mod types;
pub(crate) mod wrapper;

use chin_tools::AResult;
use deadpool_postgres::{Client, Pool, PoolError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PostgresConfig {
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
