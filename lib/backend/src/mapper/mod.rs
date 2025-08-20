pub(crate) mod db;
pub(crate) mod mappertype;

use chin_sql::{Wheres, time_type::TID};
use db::{KDb, postgres::PostgresConfig, sqlite::SqliteConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig),
}

// We decided to use `enum-dispatching` to avoid any `object-safe` questions
// and improve some speed.
pub enum MapperType {
    KDb(KDb),
}

pub(crate) trait Curd {
    fn pkey(&self) -> Wheres<'_>;
    fn tid(&self) -> TID;
}
