use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KKVQueryReq {
    pub(crate) key: String,
    pub(crate) kind: KKVType,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct KKVReqExtra {
    pub(crate) kspace: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KKVQueryRsp {
    pub(crate) value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KKVOverwriteReq {
    pub(crate) key: String,
    pub(crate) kind: String,
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
