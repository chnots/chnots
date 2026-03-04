use chin_sql::{
    GenerateTableSchema,
    str_type::{Text, Varchar},
    time_type::TID,
};
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

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct ToentTodo {
    #[gts_primary]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "i64"]
    #[gts_tosql = "todo_priority_enum_to_sql"]
    pub todo_priority: Option<TodoPriorityEnum>,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub todo_state: Option<TodoStateEnum>,

    pub todo_closed: bool,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ToentTodo {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            tid: value.try_get(Self::TID)?,
            todo_state: value.via_str_opt(Self::TODO_STATE)?,
            todo_priority: value.via_i64_opt(Self::TODO_PRIORITY)?,
            todo_closed: value.try_get(Self::TODO_CLOSED)?,
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

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for ToentEvent {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            event_defi: value.try_get(Self::EVENT_DEFI)?,
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

#[derive(Debug, Clone, Serialize, Deserialize, GenerateTableSchema)]
pub(crate) struct TodoInst {
    #[gts_primary]
    #[gts_key]
    #[gts_type = "i64"]
    pub otid: TID,

    #[gts_type = "Varchar<10>"]
    pub timezone: Option<Varchar<10>>,

    #[gts_type = "Varchar<30>"]
    pub naive_time: Varchar<30>,

    #[gts_type = "Varchar<30>"]
    #[gts_tosql = "todo_state_enum_to_sql"]
    pub target_status: Option<TodoStateEnum>,

    pub note: Option<Text>,

    #[gts_type = "i64"]
    pub alert_tid: Option<TID>,

    #[gts_primary]
    #[gts_type = "i64"]
    pub target_tid: TID,

    #[gts_unique]
    #[gts_type = "i64"]
    pub tid: TID,
}

impl TryFrom<&KDbRow> for TodoInst {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            otid: value.try_get(Self::OTID)?,
            timezone: value.try_get(Self::TIMEZONE)?,
            naive_time: value.try_get(Self::NAIVE_TIME)?,
            target_status: value.via_str_opt(Self::TARGET_STATUS)?,
            note: value.try_get(Self::NOTE)?,
            alert_tid: value.try_get(Self::ALERT_TID)?,
            target_tid: value.try_get(Self::TARGET_TID)?,
            tid: value.try_get(Self::TID)?,
        })
    }
}

impl Curd for TodoInst {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.otid, self.target_tid)
    }

    fn tid(&self) -> TID {
        self.tid
    }
}

impl_otid_support! {TodoInst}
