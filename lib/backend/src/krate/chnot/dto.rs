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
    pub head_content: Option<Text>,
    pub todo_event: Option<TodoEvent>,
    pub meta: ChnotThreadMetaFetch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchCommitReq {
    pub meta_otid: TID,
    pub kspace: Option<Varchar<40>>,
    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchCommitRsp {
    pub meta: ChnotThreadMetaFetch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMdwtCommitReqData {
    pub otid: TID,
    pub content: Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMdwtCommitReq {
    pub mdwt: ChnotMdwtCommitReqData,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotMdwtCommitRsp {
    pub todo_event: Option<TodoEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderCommitReqData {
    pub otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderCommitReq {
    pub thread_otid: TID,
    pub orders: Vec<ChnotThreadOrderCommitReqData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaCommitReqData {
    pub otid: TID,
    pub kind: ChnotKind,
    pub kind_id: Varchar<200>,
    pub kspace: Varchar<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaCommitReq {
    pub metas: Vec<ChnotMetaCommitReqData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaCommitRsp {
    pub metas: Vec<ChnotMeta>,
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
pub enum ChnotTagSearchType {
    Inset(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadListReq {
    pub query: Option<String>,
    pub thread_otid: Option<TID>,

    pub tags: Option<ChnotTagSearchType>,
    pub kinds: Vec<ChnotKind>,

    pub with_omitted: Option<bool>,
    pub with_archive: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadListRsp {
    pub data: Vec<ChnotThread>,
    pub has_next: bool,
    pub next_start: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchReq {
    pub thread_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchRsp {
    pub thread_meta: ChnotThreadMetaFetch,
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
pub struct ChnotTagListReq {
    pub query: Option<String>,
    pub tags: Option<ChnotTagSearchType>,
    pub remove_params: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotTagListRsp<T>
where
    T: Serialize + Clone,
{
    pub data: Vec<T>,
    pub start_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagUpdateReq {
    pub content: Text,
    pub thread_otid: TID,
    pub kspace: Varchar<40>,
}
