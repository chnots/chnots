use super::*;
use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::{Deserialize, Serialize};

#[derive(TryFromMultipart)]
pub(crate) struct KFileUploadReq {
    pub(crate) upload_id: String,
    pub(crate) meta_id: String,
    pub(crate) filename: String,
    pub(crate) chunk_no: usize,
    pub(crate) total_chunks: usize,
    pub(crate) chunk: FieldData<Bytes>,
    pub(crate) last_modified: i64,
    pub(crate) filesize: i64,
    pub(crate) content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct KFileUploadRsp {
    pub(crate) kfile: Option<KFileMeta>,
    pub(crate) finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InsertInlineKFileReq {
    pub(crate) meta_id: String,
    pub(crate) res: InlineKFile,
    /// archor interval second.
    pub(crate) archor_intervals: i64,
    pub(crate) filename: Option<String>,
    pub(crate) content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InsertInlineKFileRsp {
    pub(crate) true_sid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryInlineKFileReq {
    pub(crate) sid: Option<String>,
    pub(crate) meta_id: Option<String>,
    pub(crate) with_omit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryInlineKFileRsp {
    pub(crate) res: Vec<InlineKFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryKFileReq {
    pub(crate) meta_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct QueryKFileMetaRsp {
    pub(crate) meta: Option<KFileMeta>,
}

#[test]
fn tst() {
    let s = "{\"res\":{\"tid\":1749701191901010,\"rid\":\"d0d48143-f149-40eb-bb65-739559a1e2d8\",\"kspace\":\"public\",\"archor\":false,\"name\":\"1749701191901011\",\"content\":\"\",\"content_type\":\"excalidraw-v1\"},\"archor_intervals\":3600}";
    let c: Result<InsertInlineKFileReq, serde_json::Error> = serde_json::from_str(s);
    c.unwrap();
}
