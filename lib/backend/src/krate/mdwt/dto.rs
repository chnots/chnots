use std::collections::HashMap;

use chin_sql::{
    str_type::{Text, Varchar},
    time_type::TID,
};
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
pub struct MdwtCommitRsp {
    pub todo_event: Option<TodoEvent>,
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
pub enum MdwtTagSearchType {
    Inset(Vec<String>),
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
    pub kspace: Varchar<40>,
}
