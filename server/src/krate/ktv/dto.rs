use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KTVQueryReq {
    pub(crate) key: String,
    pub(crate) ttype: KTVType,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KTVQueryRsp {
    pub(crate) value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KTVOverwriteReq {
    pub(crate) key: String,
    pub(crate) ttype: String,
    pub(crate) value: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KTVOverwriteRsp {}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KTVDeleteReq {
    pub(crate) key: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KTVDeleteRsp {}
