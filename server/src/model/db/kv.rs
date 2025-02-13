use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::shared_str::SharedStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KV {
    pub key: SharedStr,
    pub value: SharedStr,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}
