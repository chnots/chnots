/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use serde::{Deserialize, Serialize};

use super::chnot::Chnot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotInsertionReq {
    pub chnot: Chnot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotInsertionRsp {}

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

    // Paging
    pub start_index: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp {
    pub data: Vec<Chnot>,
    pub next_start: i64,
    pub this_start: i64,
}
