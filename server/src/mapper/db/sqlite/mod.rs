use chin_tools::AResult;
use deadpool_sqlite::{Config, Pool, Runtime};
use serde::Deserialize;

pub(crate) mod sqltype;
pub(crate) mod wrapper;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SqliteConfig {
    filepath: String,
}

impl From<SqliteConfig> for deadpool_sqlite::Config {
    fn from(val: SqliteConfig) -> Self {
        deadpool_sqlite::Config::new(val.filepath)
    }
}

#[derive(Clone)]
pub(crate) struct Sqlite {
    pool: deadpool_sqlite::Pool,
}

impl Sqlite {
    pub(crate) fn new(config: SqliteConfig) -> AResult<Sqlite> {
        let config: Config = config.into();
        Ok(Self {
            pool: config.create_pool(Runtime::Tokio1)?,
        })
    }

    pub(crate) fn pool(&self) -> &Pool {
        &self.pool
    }
}

#[macro_export]
macro_rules! to_sqlite_params {
    ($values:expr) => {
        $values
            .iter()
            .map(|e| {
                let v: &'static dyn rusqlite::types::ToSql = e.into();
                v as &'static dyn rusqlite::types::ToSql
            })
            .collect::<Vec<&'static dyn rusqlite::types::ToSql>>()
            .as_slice()
    };
}
