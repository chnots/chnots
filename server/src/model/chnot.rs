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
pub struct Chnot {
    pub id: String,
    pub perm_id: String,
    pub content: String,
    pub r#type: ChnotType,
    pub domain: String,
    pub delete_time: Option<DateTime<FixedOffset>>,
    pub insert_time: DateTime<FixedOffset>,
    pub update_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum ChnotType {
    #[strum(serialize = "mdwt")]
    MarkdownWithToent,
}
