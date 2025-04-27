use chin_sql::{DbType, GenerateTableSql};
/// Toent: todo and event
///
/// The file mainly contains models related to todos and events.
/// Many tools attempt to handle todos and events separately,
/// but I prefer treating them as one thing.
///
/// I merged them into the word "toent."
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToentDateType {
    Chinese,
    Westen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToentType {
    Todo,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct Toent {
    #[gts_primary]
    #[gts_length = 40]
    id: String,
    #[gts_length = 40]
    chnot_id: String,
    active_flag: bool,
    #[gts_length = 500]
    original_str: String,
    #[gts_length = 500]
    #[gts_type = "String"]
    date_type: ToentDateType,
    #[gts_length = 500]
    #[gts_type = "String"]
    toent_type: ToentType,
    toent_time: DateTime<FixedOffset>,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
    insert_time: DateTime<FixedOffset>,
    update_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct ToentInst {
    #[gts_primary]
    #[gts_length = 40]
    id: String,
    #[gts_length = 40]
    toent_id: String,
    active_flag: bool,
    alert_time: DateTime<FixedOffset>,
    toent_time: DateTime<FixedOffset>,
    insert_time: DateTime<FixedOffset>,
    update_time: DateTime<FixedOffset>,
}
