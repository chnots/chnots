use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, GenerateTableSql)]
pub struct NamespaceRecord {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub name: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Deserialize, Serialize, GenerateTableSql)]
pub struct NamespaceRelation {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub sub_id: String,
    #[gts_length = 40]
    pub parent_id: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}
