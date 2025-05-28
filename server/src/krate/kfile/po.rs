use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use kdb_derives::KdbSqlInserter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct KFile {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) kspace: String,
    #[gts_length = 200]
    pub(crate) content_type: String,

    #[gts_length = 512]
    pub(crate) ori_filename: String,
    pub(crate) filesize: i64,
    pub(crate) ori_last_modified: i64,

    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql, KdbSqlInserter)]
pub(crate) struct InlineKFile {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,

    #[gts_length = 40]
    pub(crate) rid: String,
    pub(crate) archor: bool,

    #[gts_length = 200]
    pub(crate) name: String,
    pub(crate) content: String,
    pub(crate) kspace: String,

    #[gts_length = 100]
    pub(crate) content_type: String,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}
