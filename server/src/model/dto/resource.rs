use axum::extract::Multipart;
use serde::{Deserialize, Serialize};

use crate::model::db::resource::*;

pub type ResourceUploadReq = Multipart;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUploadRsp {
    pub(crate) resources: Vec<Resource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineResourceReq {
    pub res: InlineResource,
    /// archor interval second.
    pub archor_intervals: i64,
    pub ignore_conflict: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineResourceRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineResourceReq {
    pub id: Option<String>,
    pub rid: Option<String>,
    pub with_del: Option<bool>,
    pub content_type: Option<String>,
    pub name_like: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineResourceRsp {
    pub res: Vec<InlineResource>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KVQueryReq {
    pub key: String,
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
    pub key: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct KVDeleteRsp {}
