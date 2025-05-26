use actor_sqlite::{pool::ActorSqlitePool, WorkerConfig};
use chin_tools::AResult;
use serde::Deserialize;

pub(crate) mod sqltype;
pub(crate) mod wrapper;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SqliteConfig {
    filepath: String,
    pool_size: Option<u8>,
}

impl TryFrom<SqliteConfig> for ActorSqlitePool {
    type Error = anyhow::Error;

    fn try_from(value: SqliteConfig) -> Result<Self, Self::Error> {
        ActorSqlitePool::create(WorkerConfig::default().path(value.filepath))
    }
}

#[derive(Clone)]
pub(crate) struct Sqlite {
    pool: ActorSqlitePool,
}

impl Sqlite {
    pub(crate) fn new(config: SqliteConfig) -> AResult<Sqlite> {
        Ok(Self {
            pool: ActorSqlitePool::try_from(config)?
        })
    }

    pub(crate) fn pool(&self) -> &ActorSqlitePool {
        &self.pool
    }
}

