use chin_sql::str_type::Varchar;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Point2PointInfo {
    local_id: Varchar<100>,
    remote_id: Varchar<100>,
    update_time: DateTime<FixedOffset>,
}
