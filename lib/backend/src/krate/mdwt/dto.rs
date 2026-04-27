use std::collections::HashMap;

use chin_sql::{str_type::Text, time_type::TID};
use chin_tools::SharedStr;
use serde::{Deserialize, Serialize};

use crate::krate::{mdwt::MdwtRecord, toent::logic::todoevent::TodoEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtCommitReqData {
    pub otid: TID,
    pub content: Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtCommitReq {
    pub mdwt: MdwtCommitReqData,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtBlockRspData {
    pub otid: TID,
    pub title: SharedStr,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtCommitRsp {
    pub todo_event: Option<TodoEvent>,
    pub title: SharedStr,
    pub blocks: Vec<MdwtBlockRspData>,
    pub content: Option<Text>,
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
#[serde(tag = "id", content = "data")]
pub enum MdwtTagSearchType {
    Inset(Vec<String>),
}

impl MdwtTagSearchType {
    pub fn is_empty(&self) -> bool {
        match self {
            MdwtTagSearchType::Inset(items) => items.is_empty(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtTagListReq {
    pub query: Option<String>,
    pub tags: Option<MdwtTagSearchType>,
    pub remove_params: Option<bool>,

    // Paging
    pub start_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtTagListRsp<T>
where
    T: Serialize + Clone,
{
    pub data: Vec<T>,
    pub start_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtTagUpdateReq {
    pub content: Text,
    pub mdwt_otid: TID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtHistoryListReq {
    pub otid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtHistoryVersion {
    pub tid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtHistoryListRsp {
    pub versions: Vec<MdwtHistoryVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtHistoryFetchReq {
    pub otid: TID,
    pub tid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtHistoryFetchRsp {
    pub content: Option<Text>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdwtHistoryApplyReq {
    pub otid: TID,
    pub tid: TID,
}

#[derive(Debug, Clone, Serialize)]
pub struct MdwtHistoryApplyRsp {
    pub content: Option<Text>,
}
