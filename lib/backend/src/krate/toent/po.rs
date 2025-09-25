/// Toent: todo and event
///
/// The file mainly contains models related to todos and events.
/// Many tools attempt to handle todos and events separately,
/// but I prefer treating them as one thing.
///
/// I merged them into the word "toent."
use chin_sql::{
    GenerateTableSchema, SqlValue,
    str_type::{Text, Varchar},
    time_type::TID,
};

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::{
    enum_common_funcs, impl_otid_support,
    krate::toent::logic::{
        EventBuilder,
        todoevent::{TodoEvent, TodoPriorityEnum, TodoStateEnum},
    },
    mapper::{
        Curd,
        db::{KDbRow, KDbRowBehavier},
    },
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

fn todo_state_enum_to_sql(this: Option<TodoStateEnum>) -> Option<String> {
    this.map(|e| e.as_static_str().into())
}

fn todo_priority_enum_to_sql(this: Option<TodoPriorityEnum>) -> Option<i64> {
    this.map(|e| e.as_priority().into())
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct MdwtToent {
    #[gts_primary]
    #[gts_type = "i64"]
    pub mdwt_otid: TID,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    pub todo_closed: bool,

    pub note: Option<Text>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for MdwtToent {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            mdwt_otid: value.try_get(Self::MDWT_OTID)?,
            tid: value.try_get(Self::TID)?,
            todo_state: value.via_str_opt(Self::TODO_STATE)?,
            todo_priority: value.via_i64_opt(Self::TODO_PRIORITY)?,
            todo_closed: value.try_get(Self::TODO_CLOSED)?,
            note: value.try_get(Self::NOTE)?,
        })
    }
}

impl Curd for MdwtToent {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.mdwt_otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {MdwtToent}
