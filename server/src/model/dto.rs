/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use serde::{Deserialize, Serialize};

use super::chnot::Chnot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionReq {
    chnot_id: String,
    /// logic or physical deletion
    logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionRsp {}

pub struct ChnotQueryReq {
    query: String,

    // Paging
    offset: usize,
    limit: usize,
}

pub struct ChnotQueryRsp {
    result: Vec<Chnot>,
}
