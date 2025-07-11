pub(crate) mod db;
pub(crate) mod dump;
pub(crate) mod mappertype;

use chin_sql::time_type::TID;
use chin_tools::EResult;
use db::{postgres::PostgresConfig, sqlite::SqliteConfig, KDb};
use dump::RecordCallbackType;
use serde::Deserialize;

use crate::{mapper::db::KDbRow, model::omit_tid::OmitTID};

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

#[derive(Debug, Deserialize, Clone)]
pub struct TheSameKey {
    pub id: String,
    pub omit_tid: OmitTID,
    pub tid: TID,
}