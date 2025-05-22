use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};

use crate::model::db::kfile::*;

#[derive(TryFromMultipart)]
pub(crate) struct KFileUploadReq {
    pub(crate) res_id: String,
    pub(crate) filename: String,
    pub(crate) chunk_no: usize,
    pub(crate) total_chunks: usize,
    pub(crate) chunk: FieldData<Bytes>,
    pub(crate) last_modified: i64,
    pub(crate) filesize: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KFileUploadRsp {
    pub(crate) kfile: Option<KFile>,
    pub(crate) finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InsertInlineKFileReq {
    pub(crate) res: InlineKFile,
    /// archor interval second.
    pub(crate) archor_intervals: i64,
    pub(crate) ignore_conflict: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InsertInlineKFileRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryInlineKFileReq {
    pub(crate) id: Option<String>,
    pub(crate) rid: Option<String>,
    pub(crate) with_del: Option<bool>,
    pub(crate) content_type: Option<String>,
    pub(crate) name_like: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryInlineKFileRsp {
    pub(crate) res: Vec<InlineKFile>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryKFileReq {
    pub(crate) id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryKFileRsp {
    pub(crate) res: Option<KFile>,
}


#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KVQueryReq {
    pub(crate) key: String,
    pub(crate) ttype: KVType,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KVQueryRsp {
    pub(crate) value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KVOverwriteReq {
    pub(crate) key: String,
    pub(crate) ttype: String,
    pub(crate) value: String
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KVOverwriteRsp {}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct KVDeleteReq {
    pub(crate) key: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct KVDeleteRsp {}
