use chrono::{DateTime, FixedOffset};

pub struct NamespaceRecord {
    pub id: String,
    pub name: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

pub struct NamespaceRelation {
    pub id: String,
    pub sub_id: String,
    pub parent_id: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}
