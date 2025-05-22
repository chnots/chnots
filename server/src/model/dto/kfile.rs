use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};

use crate::model::db::kfile::*;

#[derive(TryFromMultipart)]
pub struct KFileUploadReq {
    pub res_id: String,
    pub filename: String,
    pub chunk_no: usize,
    pub total_chunks: usize,
    pub chunk: FieldData<Bytes>,
    pub last_modified: i64,
    pub filesize: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFileUploadRsp {
    pub(crate) kfile: Option<KFile>,
    pub(crate) finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineKFileReq {
    pub res: InlineKFile,
    /// archor interval second.
    pub archor_intervals: i64,
    pub ignore_conflict: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineKFileRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineKFileReq {
    pub id: Option<String>,
    pub rid: Option<String>,
    pub with_del: Option<bool>,
    pub content_type: Option<String>,
    pub name_like: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineKFileRsp {
    pub res: Vec<InlineKFile>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryKFileReq {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryKFileRsp {
    pub res: Option<KFile>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct KVQueryReq {
    pub key: String,
    pub ttype: KVType,
}

#[derive(Clone, Debug, Serialize)]
pub struct KVQueryRsp {
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KVOverwriteReq {
    pub key: String,
    pub ttype: String,
    pub value: String
}

#[derive(Clone, Debug, Serialize)]
pub struct KVOverwriteRsp {}

#[derive(Clone, Debug, Deserialize)]
pub struct KVDeleteReq {
    pub key: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct KVDeleteRsp {}
