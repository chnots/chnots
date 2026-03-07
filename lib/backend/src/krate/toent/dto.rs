use chin_sql::time_type::TID;
use chin_tools::score::PossibleScore;
use serde::{Deserialize, Serialize};

use crate::krate::mdwt::MdwtTagSearchType;
use crate::krate::toent::logic::todoevent::{TodoPriorityEnum, TodoStateEnum};

#[derive(Clone, Debug, Deserialize)]
pub struct ToentGuessReq {
    pub input: String,
}

#[derive(Clone, Debug)]
pub struct GuessElem<T> {
    pub timestamp: T,
    pub score: PossibleScore,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentGuessRsp<T> {
    pub toents: Vec<T>,
}

impl<T> From<(T, PossibleScore)> for GuessElem<T> {
    fn from(value: (T, PossibleScore)) -> Self {
        GuessElem {
            timestamp: value.0,
            score: value.1,
        }
    }
}

pub fn toent2<E, V>(value: GuessElem<E>) -> GuessElem<V>
where
    E: Into<V>,
{
    GuessElem {
        timestamp: value.timestamp.into(),
        score: value.score,
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentInstListReq {
    pub start_date: String,
    pub end_date: String,
    pub include_completed: bool,
    pub include_uncompleted: bool,
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentInstCountReq {
    pub start_date: String,
    pub end_date: String,
    pub include_completed: bool,
    pub include_uncompleted: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentSearchReq {
    pub start_date: String,
    pub end_date: String,
    pub include_completed: bool,
    pub include_uncompleted: bool,
    pub query: Option<String>,
    pub tags: Option<MdwtTagSearchType>,
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentTodoStateCommitReq {
    pub otid: TID,
    pub todo_state: TodoStateEnum,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentTodoStateCommitRsp {
    pub otid: TID,
    pub todo_state: TodoStateEnum,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentInstListRsp {
    pub items: Vec<ToentScheduleItemDto>,
    pub has_next: bool,
    pub next_start: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentInstCountRsp {
    pub total: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentScheduleItemDto {
    pub inst: TodoInstDto,
    pub todo: Option<ToentTodoDto>,
    pub event: Option<ToentEventDto>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentTodoDto {
    pub otid: TID,
    pub todo_priority: Option<TodoPriorityEnum>,
    pub todo_state: Option<TodoStateEnum>,
    pub todo_closed: bool,
    pub tid: TID,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentEventDefiItemDto {
    pub raw: String,
    pub standard: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentEventDefiDto {
    pub events: Vec<ToentEventDefiItemDto>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentEventDto {
    pub otid: TID,
    pub event_defi: ToentEventDefiDto,
    pub tid: TID,
}

#[derive(Clone, Debug, Serialize)]
pub struct TodoInstDto {
    pub otid: TID,
    pub timezone: Option<String>,
    pub naive_time: String,
    pub target_status: Option<TodoStateEnum>,
    pub note: Option<String>,
    pub alert_tid: Option<TID>,
    pub target_tid: TID,
    pub tid: TID,
}
