use std::collections::HashMap;

use chin_sql::{
    str_type::{Text, Varchar},
    time_type::TID,
};
use serde::{Deserialize, Serialize};

use crate::krate::toent::logic::todoevent::TodoEvent;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    pub head_record: MdwtRecord,
    pub meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMetaReq {
    pub meta_otid: TID,
    pub kspace: Option<Varchar<40>>,
    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteMetaRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteRecordReqMdwt {
    pub block_otid: TID,
    pub content: Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteBlockReqMeta {
    pub block_otid: TID,
    pub korder: i64,
    pub kind: ChnotKind,
    pub kind_id: Varchar<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteBlockReq {
    pub meta_otid: TID,
    pub mdwts: Vec<ChnotOverwriteRecordReqMdwt>,
    pub metas: Vec<ChnotOverwriteBlockReqMeta>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChnotOverwriteRecordRsp {
    pub todo_event: Option<TodoEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotArchiveReq {
    pub meta_otid: TID,
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
    pub meta_otid: Option<TID>,
    pub record_otid: Option<TID>,

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
pub struct ChnotMetaReq {
    pub chnot_meta_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotMetaRsp {
    pub chnot_meta: ChnotMetadata,
    pub block_meta_sorted: Vec<ChnotBlockMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtBlocksReq {
    pub mdwt_otids: Vec<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtBlocksRsp {
    pub mdwt_map: HashMap<TID, MdwtRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Toents {
    pub toent_inst_map: HashMap<TID, Vec<ChnotBlockToent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagQueryReq {
    pub query: Option<String>,
    pub tags: Option<ChnotTagSearchType>,
    pub remove_params: Option<bool>,
    pub mdwt_map: HashMap<TID, MdwtRecord>,

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
    pub meta_otid: TID,
    pub kspace: Varchar<40>,
}
