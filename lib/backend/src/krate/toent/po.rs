/// Toent: todo and event
///
/// The file mainly contains models related to todos and events.
/// Many tools attempt to handle todos and events separately,
/// but I prefer treating them as one thing.
///
/// I merged them into the word "toent."
use chin_sql::{GenerateTableSchema, SqlValue, str_type::Varchar, time_type::TID};

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::{
    enum_common_funcs,
    krate::toent::logic::{EventBuilder, todoevent::TodoEvent},
};

#[derive(Debug, Clone, Sequence)]
pub(crate) enum TimeEventAction {
    Skip,
}

impl TimeEventAction {
    pub(crate) fn as_static_str(&self) -> &'static str {
        match self {
            TimeEventAction::Skip => "skip",
        }
    }
}

enum_common_funcs! {TimeEventAction}

fn time_event_action_to_sql(this: TimeEventAction) -> Varchar<40> {
    this.as_static_str().try_into().unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentTimeEventInst {
    #[gts_key]
    #[gts_type = "i64"]
    chnot_otid: TID,

    #[gts_primary]
    chnot_block_id: Varchar<100>,

    #[gts_primary]
    target_time_utc: i64,

    alert_time_utc: i64,

    #[gts_type = "Varchar<40>"]
    #[gts_tosql = "time_event_action_to_sql"]
    action: TimeEventAction,

    #[gts_unique]
    #[gts_type = "i64"]
    tid: TID,
}

impl<'a> From<TodoEvent> for SqlValue<'a> {
    fn from(val: TodoEvent) -> Self {
        let s = val.standard_string().to_string();
        SqlValue::Str(s.into())
    }
}
