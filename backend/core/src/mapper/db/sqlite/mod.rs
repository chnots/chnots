use actor_sqlite::{pool::ActorSqlitePool, pool_config::PoolConfig};
use chin_tools::AResult;
use serde::Deserialize;

pub(crate) mod wrapper;

#[derive(Debug, Deserialize, Clone)]
pub struct SqliteConfig {
    pub filepath: String,
    pub pool_size: Option<u8>,
}

impl TryFrom<SqliteConfig> for ActorSqlitePool {
    type Error = anyhow::Error;

    fn try_from(value: SqliteConfig) -> Result<Self, Self::Error> {
        Ok(ActorSqlitePool::try_from(
            PoolConfig::default()
                .path(value.filepath)
                .pool_size(value.pool_size.unwrap_or(4)),
        )?)
    }
}

#[derive(Clone)]
pub(crate) struct Sqlite {
    pool: ActorSqlitePool,
}

impl Sqlite {
    pub(crate) fn new(config: SqliteConfig) -> AResult<Sqlite> {
        Ok(Self {
            pool: ActorSqlitePool::try_from(config)?,
        })
    }
}
