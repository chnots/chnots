pub(crate) mod db;
pub(crate) mod mappertype;

use chin_sql::time_type::TID;
use db::{postgres::PostgresConfig, sqlite::SqliteConfig, KDb};
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

#[derive(Debug, Deserialize, Clone)]
pub struct TheSameKey {
    pub id: String,
    pub omit_tid: OmitTID,
    pub tid: TID,
}