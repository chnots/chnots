use super::*;
use chin_tools::SharedStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KKVQueryOneReq {
    pub(crate) key: String,
    pub(crate) kind: KKVType,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KKVQueryManyReq {
    pub(crate) key: Option<String>,
    pub(crate) kind: Option<KKVType>,
    pub(crate) kspace: Option<SharedStr>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KKVQueryOneRsp {
    pub(crate) value: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KKVQueryManyRsp {
    pub(crate) kkvs: Vec<KKV>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KKVOverwriteReq {
    pub(crate) key: String,
    pub(crate) kind: KKVType,
    pub(crate) value: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KKVOverwriteRsp {}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KKVDeleteReq {
    pub(crate) key: String,
    pub(crate) kind: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KKVDeleteRsp {}
