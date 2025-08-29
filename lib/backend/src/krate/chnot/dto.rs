use std::collections::HashMap;

use chin_sql::{
    str_type::{Text, Varchar},
    time_type::TID,
};
use serde::{Deserialize, Serialize};

use crate::krate::toent::logic::todoevent::TodoEvent;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThread {
    pub head_record: MdwtRecord,
    pub meta: ChnotThreadMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteThreadMetaReq {
    pub meta_otid: TID,
    pub kspace: Option<Varchar<40>>,
    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteThreadMetaRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMdwtReqData {
    pub otid: TID,
    pub content: Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMdwtReq {
    pub thread_otid: TID,
    pub mdwts: Vec<ChnotOverwriteMdwtReqData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMetaReqData {
    pub otid: TID,
    pub korder: i64,
    pub kind: ChnotKind,
    pub kind_id: Varchar<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMetaReq {
    pub thread_otid: TID,
    pub metas: Vec<ChnotOverwriteMetaReqData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMetaRsp {}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotOverwriteMdwtRsp {
    pub todo_event: Option<TodoEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadArchiveReq {
    pub thread_otid: TID,
    /// logic or physical deletion
    pub logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadArchiveRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChnotThreadTagSearchType {
    Inset(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadQueryReq {
    pub query: Option<String>,
    pub thread_otid: Option<TID>,

    pub tags: Option<ChnotThreadTagSearchType>,
    pub kinds: Vec<ChnotKind>,

    pub with_omitted: Option<bool>,
    pub with_archive: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadQueryRsp {
    pub data: Vec<ChnotThread>,
    pub has_next: bool,
    pub next_start: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaReq {
    pub thread_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaRsp {
    pub thread_meta: ChnotThreadMeta,
    pub chnot_meta_sorted: Vec<ChnotMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtRecordsReq {
    pub mdwt_otids: Vec<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtRecordsRsp {
    pub mdwt_map: HashMap<TID, MdwtRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toents {
    pub toent_inst_map: HashMap<TID, Vec<ChnotToent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadTagQueryReq {
    pub query: Option<String>,
    pub tags: Option<ChnotThreadTagSearchType>,
    pub remove_params: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotThreadTagQueryRsp<T>
where
    T: Serialize + Clone,
{
    pub data: Vec<T>,
    pub start_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadTagUpdateReq {
    pub content: Text,
    pub thread_otid: TID,
    pub kspace: Varchar<40>,
}
