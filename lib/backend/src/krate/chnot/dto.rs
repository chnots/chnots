use chin_sql::{str_type::Varchar, time_type::TID};
use serde::{Deserialize, Serialize};

use crate::krate::mdwt::MdwtTagSearchType;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotSearchRspData {
    pub title: Option<String>,
    pub meta: ChnotMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderCommitReqData {
    pub otid: TID,
    pub closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderCommitReq {
    pub thread_otid: TID,
    pub orders: Vec<ChnotThreadOrderCommitReqData>,
    pub remove_others: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaCommitReqData {
    pub otid: TID,
    pub kind: ChnotKind,
    pub kspace: Varchar<40>,
    pub archive: Option<bool>,
    pub pin_it: Option<bool>,
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
pub struct ChnotMetaListReq {
    pub otids: Vec<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaListRsp {
    pub metas: Vec<ChnotMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotSearchReq {
    pub query: Option<String>,

    pub tags: Option<MdwtTagSearchType>,
    pub kinds: Vec<ChnotKind>,

    with_archive: Option<bool>,
    hide_thread: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

impl ChnotSearchReq {
    pub fn with_archive(&self) -> bool {
        self.with_archive.unwrap_or(false)
    }

    pub fn hide_thread(&self) -> bool {
        self.hide_thread.unwrap_or(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchReq {
    pub otid: TID,
    pub include_hist: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchRspData {
    pub meta: ChnotMeta,
    pub closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadMetaFetchRsp {
    pub chnot_meta_sorted: Vec<ChnotThreadMetaFetchRspData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderArchiveRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotThreadOrderArchiveReq {
    pub thread_otid: TID,
    pub otids: Vec<TID>,
}
