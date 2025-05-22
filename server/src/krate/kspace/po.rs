
use chin_sql::{DbType, GenerateTableSql};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, GenerateTableSql)]
pub(crate) struct WorkspaceRecord {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) name: String,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Deserialize, Serialize, GenerateTableSql)]
pub(crate) struct WorkspaceRelation {
    #[gts_primary]
    #[gts_length = 40]
    pub(crate) id: String,
    #[gts_length = 40]
    pub(crate) sub_id: String,
    #[gts_length = 40]
    pub(crate) parent_id: String,
    pub(crate) delete_time: Option<DateTime<FixedOffset>>,
    pub(crate) update_time: Option<DateTime<FixedOffset>>,
    pub(crate) insert_time: DateTime<FixedOffset>,
}
