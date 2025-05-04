use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql)]
pub struct Resource {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub namespace: String,
    #[gts_length = 512]
    pub ori_filename: String,
    #[gts_length = 200]
    pub content_type: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Clone, Serialize, Deserialize, Debug, GenerateTableSql)]
pub struct InlineResource {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,

    #[gts_length = 40]
    pub rid: String,
    pub archor: bool,

    #[gts_length = 200]
    pub name: String,
    pub content: String,
    pub namespace: String,

    #[gts_length = 100]
    pub content_type: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct KV {
    #[gts_primary]
    #[gts_length = 500]
    pub key: String,
    pub value: String,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}
