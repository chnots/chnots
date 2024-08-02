/// Chnot: knot, which stands for the note.
///
/// Ancients used knots to record events,
/// so I use "knot" as the basic unit for my notebook,
/// but the name "knot" is too repetitive, so I made a change.
/// 
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChnotType {
    MarkdownWithToent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    id: String,
    perm_id: String,
    content: String,
    r#type: ChnotType,
    domain: String,
    delete_time: DateTime<FixedOffset>,
    insert_time: DateTime<FixedOffset>,
    update_time: DateTime<FixedOffset>,
}
