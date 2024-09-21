/// Chnot: knot, which stands for the note.
///
/// Ancients used knots to record events,
/// so I use "knot" as the basic unit for my notebook,
/// but the name "knot" is too repetitive, so I made a change.
///
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::Display;
use strum_macros::EnumString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotRecord {
    pub id: String,
    pub meta_id: String,
    pub content: String,
    pub omit_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetadata {
    pub id: String,
    pub namespace: String,
    pub kind: String,
    pub pin_time: Option<DateTime<FixedOffset>>,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub update_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum ChnotKind {
    #[strum(serialize = "mdwt")]
    #[serde(rename = "mdwt")]
    MarkdownWithToent,
}
