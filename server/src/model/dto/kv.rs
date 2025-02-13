use serde::{Deserialize, Serialize};

use crate::model::{db::kv::KV, shared_str::SharedStr};

#[derive(Clone, Debug, Deserialize)]
pub struct QueryKVReq {
    pub key: SharedStr,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryKVRsp {
    pub kv: Option<KV>
}

#[derive(Clone, Debug, Deserialize)]
pub struct OverwriteKVReq {
    pub kv: KV
}

#[derive(Clone, Debug, Serialize)]
pub struct OverwriteKVRsp {
}