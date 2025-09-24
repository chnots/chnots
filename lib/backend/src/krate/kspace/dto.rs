use chin_sql::str_type::Varchar;
use serde::{Deserialize, Serialize};

use super::KSpace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceListReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceListRsp {
    pub kspaces: Vec<KSpace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceCommitReq {
    pub kspace: KSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceCommitRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceArchiveReq {
    pub kspace_name: Varchar<500>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceArchiveRsp {}
