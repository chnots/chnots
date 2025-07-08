pub(crate) mod db;
pub(crate) mod dump;
pub(crate) mod mappertype;

use chin_tools::EResult;
use db::{postgres::PostgresConfig, sqlite::SqliteConfig, KDb};
use dump::RecordCallbackType;
use serde::Deserialize;

use crate::mapper::db::KDbRow;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig),
}

pub enum MapperType {
    KDb(KDb),
}

pub enum MapperRowType {
    KDb(KDbRow)
}

pub(crate) trait DumpMapper {
    type RowType<'a>;

    async fn dump_and_callback(&self, callback: &RecordCallbackType) -> EResult;
}
