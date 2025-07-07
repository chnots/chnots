use super::*;
use chin_sql::{str_type::{Text, Varchar}, time_type::TID};
use chin_tools::SharedStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct KKVQueryOneReq {
    pub key: Varchar<500>,
    pub kind: Varchar<100>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KKVQueryManyReq {
    pub key: Option<Varchar<500>>,
    pub kind: Option<Varchar<100>>,
    pub kspace: Option<SharedStr>,
}

#[derive(Clone, Debug, Serialize)]
pub struct KKVQueryOneRsp {
    pub value: Option<Text>,
    pub(crate) tid: Option<TID>,
}

#[derive(Clone, Debug, Serialize)]
pub struct KKVQueryManyRsp {
    pub kkvs: Vec<KKV>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KKVOverwriteReq {
    pub key: Varchar<500>,
    pub kind: Varchar<100>,
    pub value: Text,
}

#[derive(Clone, Debug, Serialize)]
pub struct KKVOverwriteRsp {}

#[derive(Clone, Debug, Deserialize)]
pub struct KKVDeleteReq {
    pub key: String,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct KKVDeleteRsp {}
