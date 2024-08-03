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
    pub query: String,

    // Paging
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp {
    pub result: Vec<Chnot>,
}
