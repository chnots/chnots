use chin_sql::{SqlValue, time_type::TID};
/// Toent: todo and event
///
/// The file mainly contains models related to todos and events.
/// Many tools attempt to handle todos and events separately,
/// but I prefer treating them as one thing.
///
/// I merged them into the word "toent."
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::krate::toent::logic::{EventBuilder, todoevent::TodoEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ToentDateType {
    Chinese,
    Westen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ToentType {
    Todo,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Toent {
    tid: TID,
    chnot_id: TID,
    active_flag: bool,
    original_str: String,
    date_type: ToentDateType,
    toent_type: ToentType,
    toent_time: DateTime<FixedOffset>,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
    insert_time: DateTime<FixedOffset>,
    update_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ToentInst {
    tid: TID,
    toent_id: TID,
    active_flag: bool,
    alert_time: DateTime<FixedOffset>,
    toent_time: DateTime<FixedOffset>,
    insert_time: DateTime<FixedOffset>,
    update_time: DateTime<FixedOffset>,
}

impl<'a> From<TodoEvent> for SqlValue<'a> {
    fn from(val: TodoEvent) -> Self {
        let s = val.standard_string().to_string();
        SqlValue::Str(s.into())
    }
}
