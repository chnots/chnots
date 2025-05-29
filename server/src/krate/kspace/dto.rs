use serde::{Deserialize, Serialize};

use super::KSpace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KSpaceQueryAllReq {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KSpaceQueryAllRsp {
    pub(crate) kspaces: Vec<KSpace>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KSpaceOverwriteReq {
    pub(crate) kspace: KSpace
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KSpaceOverwriteRsp {}