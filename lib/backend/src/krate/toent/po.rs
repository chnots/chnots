/// Toent: todo and event
///
/// The file mainly contains models related to todos and events.
/// Many tools attempt to handle todos and events separately,
/// but I prefer treating them as one thing.
///
/// I merged them into the word "toent."
use chin_sql::{GenerateTableSchema, SqlValue, str_type::Varchar, time_type::TID};

use serde::{Deserialize, Serialize};

use crate::krate::toent::logic::{EventBuilder, todoevent::TodoEvent};

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentTimeEventInst {
    chnot_block_id: Varchar<100>,

    target_time_utc: i64,
    alert_time_utc: i64,

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
