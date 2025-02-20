use chin_tools::shared_str::SharedStr;
use serde::{Deserialize, Serialize};

use crate::model::db::kv::KV;

#[derive(Clone, Debug, Deserialize)]
pub struct KVQueryReq {
    pub key: SharedStr,
}

#[derive(Clone, Debug, Serialize)]
pub struct KVQueryRsp {
    pub kv: Option<KV>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KVOverwriteReq {
    pub kv: KV,
}

#[derive(Clone, Debug, Serialize)]
pub struct KVOverwriteRsp {}


#[derive(Clone, Debug, Deserialize)]
pub struct KVDeleteReq {
    pub key: SharedStr
}

#[derive(Clone, Debug, Serialize)]
pub struct KVDeleteRsp {}
