use std::collections::HashMap;

use chin_sql::{
    str_type::{Text, Varchar},
    time_type::TID,
};
use serde::{Deserialize, Serialize};

use crate::krate::{
    mdwt::MdwtTagSearchType,
    toent::{logic::todoevent::TodoEvent, po::MdwtToent},
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThread {
    pub head_content: Option<Text>,
    pub todo_event: Option<TodoEvent>,
    pub meta: ChnotThreadMeta,
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
    pub meta: ChnotThreadMeta,
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
pub struct ChnotThreadListReq {
    pub query: Option<String>,
    pub thread_otid: Option<TID>,

    pub tags: Option<MdwtTagSearchType>,
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
    pub thread_meta: ChnotThreadMeta,
    pub chnot_meta_sorted: Vec<ChnotMeta>,
}
