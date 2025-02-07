use std::ops::Deref;

use crate::toent::PossibleToent;
use serde::{Deserialize, Serialize};

use crate::model::db::chnot::{ChnotKind, ChnotMetadata, ChnotRecord};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    pub record: ChnotRecord,
    pub meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateReq {
    pub chnot_meta_id: String,

    pub update_time: bool,

    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteReq {
    pub chnot: ChnotRecord,
    pub kind: ChnotKind,
}

impl Deref for ChnotOverwriteReq {
    type Target = ChnotRecord;

    fn deref(&self) -> &Self::Target {
        &self.chnot
    }
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

    pub with_deleted: Option<bool>,
    pub with_omitted: Option<bool>,

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
