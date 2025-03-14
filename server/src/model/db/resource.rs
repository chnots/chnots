use chin_tools::SharedStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: String,
    pub namespace: String,
    pub ori_filename: String,
    pub content_type: String,
    pub delete_time: Option<DateTime<Utc>>,
    pub insert_time: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineResource {
    pub id: String,
    pub name: String,
    pub content: SharedStr,
    pub content_type: String,
    pub delete_time: Option<DateTime<Utc>>,
    pub insert_time: DateTime<Utc>,
}
