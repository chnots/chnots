use chin_sql::DbType;
use chin_sql::GenerateTableSql;
/// Chnot: knot, which stands for the note.
///
/// Ancients used knots to record events,
/// so I use "knot" as the basic unit for my notebook,
/// but the name "knot" is too repetitive, so I made a change.
///
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use strum::AsStaticStr;
use strum::Display;
use strum::IntoStaticStr;
use strum_macros::EnumString;

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct ChnotRecord {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub meta_id: String,
    pub content: String,
    pub omit_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct ChnotMetadata {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub namespace: String,
    #[gts_length = 40]
    pub kind: String,
    pub pin_time: Option<DateTime<FixedOffset>>,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
    pub archive_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSql)]
pub struct ChnotTag {
    #[gts_primary]
    #[gts_length = 40]
    pub id: String,
    #[gts_length = 40]
    pub namespace: String,
    #[gts_length = 800]
    pub tag: String,
    #[gts_type = "i32"]
    pub category: ChnotTagType,
    pub chnot_meta_id: String,
    pub insert_time: DateTime<FixedOffset>,
}

impl AsRef<str> for ChnotTag {
    fn as_ref(&self) -> &str {
        &self.tag
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, AsRefStr)]
pub enum ChnotKind {
    #[strum(serialize = "mdwt")]
    #[serde(rename = "mdwt")]
    MarkdownWithToent,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum ChnotTagType {
    Dir = 99,
    ParentDir = 98,
    Common = 1,
}
