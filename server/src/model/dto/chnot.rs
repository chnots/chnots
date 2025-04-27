use crate::{model::db::chnot::ChnotTag, toent::PossibleToent};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::model::db::chnot::{ChnotKind, ChnotMetadata, ChnotRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    pub record: ChnotRecord,
    pub meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateReq {
    pub meta_id: String,

    pub namespace: Option<String>,

    pub update_time: bool,

    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteReq {
    pub id: Option<String>,
    pub meta_id: Option<String>,
    pub content: String,
    pub kind: ChnotKind,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteRsp {
    pub chnot: Chnot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionReq {
    pub chnot_id: String,
    /// logic or physical deletion
    pub logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryReq {
    pub query: Option<String>,
    pub meta_id: Option<String>,
    pub record_id: Option<String>,
    pub tag_keyword: Option<String>,

    pub with_deleted: Option<bool>,
    pub with_omitted: Option<bool>,
    pub with_archived: Option<bool>,

    // Paging
    pub start_index: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp<T> {
    pub data: T,
    pub start_index: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentGuessReq {
    pub input: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentGuessRsp {
    pub toents: Vec<PossibleToent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagQueryReq {
    pub query: Option<String>,

    // Paging
    pub start_index: u64,
    pub page_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagQueryRsp {
    pub data: Vec<ChnotTag>,

    pub start_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotTagNamesRsp {
    pub data: Vec<String>,

    pub start_index: u64,
}
