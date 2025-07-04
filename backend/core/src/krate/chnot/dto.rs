use chin_sql::{str_type::{Text, Varchar}, time_type::TID};
use serde::{Deserialize, Serialize};

use crate::krate::toent::logic::todoevent::TodoEvent;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    pub record: ChnotRecord,
    pub meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateReq {
    pub meta_tid: TID,
    pub kspace: Option<Varchar<40>>,
    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteReq {
    pub meta_tid: Option<TID>,
    pub content: Text,
    pub kind: ChnotKind,
    pub kind_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteRsp {
    pub meta_tid: TID,
    pub rec_tid: TID,
    pub kspace: Varchar<40>,
    pub archor: bool,
    pub todo_event: Option<TodoEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotArchiveReq {
    pub meta_tid: TID,
    /// logic or physical deletion
    pub logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotArchiveRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChnotTagSearchType {
    Inset(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryReq {
    pub query: Option<String>,
    pub meta_tid: Option<TID>,
    pub record_tid: Option<TID>,

    pub tags: Option<ChnotTagSearchType>,
    pub kinds: Vec<ChnotKind>,

    pub with_omitted: Option<bool>,
    pub with_archive: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp<T> {
    pub data: Vec<T>,
    pub has_next: bool,
    pub next_start: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagQueryReq {
    pub query: Option<String>,
    pub tags: Option<ChnotTagSearchType>,
    pub remove_params: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotTagQueryRsp<T>
where
    T: Serialize + Clone,
{
    pub data: Vec<T>,

    pub start_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagUpdateReq {
    pub content: Text,
    pub meta_tid: TID,
    pub kspace: Varchar<40>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotKindRelQueryReq {
    pub meta_tid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotKindRelQueryRsp {
    pub kind_rel: ChnotKindRel,
}
