use std::collections::HashMap;

use chin_sql::time_type::TID;
use chin_tools::score::PossibleScore;
use serde::{Deserialize, Serialize};

use crate::krate::mdwt::MdwtTagSearchType;
use crate::krate::toent::logic::todoevent::TodoStateEnum;
use crate::krate::toent::po::{ToentEventDefi, ToentInst};
use crate::krate::toent::todoevent::TodoPriorityEnum;

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
pub struct ToentInstCountReq {
    pub start_tid: TID,
    pub end_tid: TID,
    pub include_completed: bool,
    pub include_uncompleted: bool,
    #[serde(default)]
    pub include_no_time_todo: bool,
    #[serde(default)]
    pub ranges: Vec<ToentInstCountRangeReq>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentInstCountRangeReq {
    pub key: String,
    pub start_tid: TID,
    pub end_tid: TID,
    #[serde(default)]
    pub include_no_time_todo: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentSearchReq {
    pub start_tid: TID,
    pub end_tid: TID,
    pub include_completed: bool,
    pub include_uncompleted: bool,
    #[serde(default)]
    pub include_no_time_todo: bool,
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
pub struct ToentSearchRsp {
    pub items: Vec<ToentSearchRspData>,
    pub has_next: bool,
    pub next_start: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentInstCountRsp {
    pub total: usize,
    pub totals: HashMap<String, usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentSearchRspData {
    pub inst: ToentInst,
    pub defi: Option<ToentEventDefi>,
    pub title: String,
}
