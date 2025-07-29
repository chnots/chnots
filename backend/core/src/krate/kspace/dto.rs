use chin_sql::str_type::Varchar;
use serde::{Deserialize, Serialize};

use super::KSpace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceQueryAllReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceQueryAllRsp {
    pub kspaces: Vec<KSpace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceOverwriteReq {
    pub kspace: KSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceOverwriteRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceDeletionReq {
    pub kspace_name: Varchar<500>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSpaceDeletionRsp {}
