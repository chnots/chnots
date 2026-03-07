use chin_sql::{GenerateTableSchema, str_type::Text, time_type::TID};
use serde::{Deserialize, Serialize};

use crate::{
    impl_otid_support,
    krate::toent::logic::todoevent::{TodoPriorityEnum, TodoStateEnum},
    mapper::{
        Curd,
        db::{KDbRow, KDbRowBehavier},
    },
};

fn todo_state_enum_to_sql(this: Option<TodoStateEnum>) -> Option<String> {
    this.map(|e| e.as_static_str().into())
}

fn todo_priority_enum_to_sql(this: Option<TodoPriorityEnum>) -> Option<i64> {
    this.map(|e| e.as_priority().into())
}

fn timezone_to_sql(this: Option<isize>) -> Option<i64> {
    this.map(|e| e as i64)
}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentTodo {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    #[gts_type = "i64"]
    pub alert_tid: Option<TID>,

    #[gts_type = "i64"]
    pub start_tid: Option<TID>,

    #[gts_type = "i64"]
    pub end_tid: Option<TID>,

    #[gts_type = "i64"]
    #[gts_tosql = "timezone_to_sql"]
    pub timezone: Option<isize>,

    pub closed: Option<bool>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,

    pub note: Option<Text>,
}

impl TryFrom<&KDbRow> for ToentTodo {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let timezone: Option<i64> = value.try_get(Self::TIMEZONE)?;
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            tid: value.try_get(Self::TID)?,
            todo_state: value.via_str_opt(Self::TODO_STATE)?,
            todo_priority: value.via_i64_opt(Self::TODO_PRIORITY)?,
            alert_tid: value.try_get(Self::ALERT_TID)?,
            start_tid: value.try_get(Self::START_TID)?,
            end_tid: value.try_get(Self::END_TID)?,
            timezone: timezone.map(|v| v as isize),
            closed: value.try_get(Self::CLOSED)?,
            note: value.try_get(Self::NOTE)?,
        })
    }
}

impl Curd for ToentTodo {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ToentTodo}

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentEvent {
    #[gts_primary]
    #[gts_key]
    #[gts_type = "i64"]
    pub otid: TID,

    pub event_defi: Text,

    #[gts_type = "i64"]
    pub start_time: Option<TID>,

    #[gts_type = "i64"]
    #[gts_tosql = "timezone_to_sql"]
    pub start_timezone: Option<isize>,

    #[gts_type = "i64"]
    pub end_time: Option<TID>,

    #[gts_type = "i64"]
    #[gts_tosql = "timezone_to_sql"]
    pub end_timezone: Option<isize>,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ToentEvent {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let start_timezone: Option<i64> = value.try_get(Self::START_TIMEZONE)?;
        let end_timezone: Option<i64> = value.try_get(Self::END_TIMEZONE)?;
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            event_defi: value.try_get(Self::EVENT_DEFI)?,
            start_time: value.try_get(Self::START_TIME)?,
            start_timezone: start_timezone.map(|v| v as isize),
            end_time: value.try_get(Self::END_TIME)?,
            end_timezone: end_timezone.map(|v| v as isize),
            tid: value.try_get(Self::TID)?,
        })
    }
}

impl Curd for ToentEvent {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {ToentEvent}
