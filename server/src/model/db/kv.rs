use chin_sql::DbType;
use chin_sql::GenerateTableSql;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct KV {
    #[gts_primary]
    #[gts_length = 500]
    pub key: String,
    pub value: String,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}
