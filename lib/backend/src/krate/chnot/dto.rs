use chin_sql::{str_type::Varchar, time_type::TID};
use serde::{Deserialize, Serialize};

use crate::krate::mdwt::MdwtTagSearchType;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadListRspData {
    pub preview_text: Option<String>,
    pub chnot_otid: Option<String>,
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
    pub kspace: Varchar<40>,
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
pub struct ChnotThreadListReq {
    pub query: Option<String>,

    pub tags: Option<MdwtTagSearchType>,
    pub kinds: Vec<ChnotKind>,

    pub with_archive: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadListRsp {
    pub data: Vec<ChnotThreadListRspData>,
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
