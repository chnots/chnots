use super::*;
use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chin_sql::str_type::Varchar;
use serde::{Deserialize, Serialize};

#[derive(TryFromMultipart)]
pub struct KFileUploadReq {
    pub upload_id: String,
    pub meta_id: String,
    pub filename: String,
    pub chunk_no: usize,
    pub total_chunks: usize,
    pub chunk: FieldData<Bytes>,
    pub last_modified: i64,
    pub filesize: i64,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFileUploadRsp {
    pub kfile: Option<KFileMeta>,
    pub finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineKFileReq {
    pub meta_id: Varchar<100>,
    pub res: InlineKFile,
    /// archor interval second.
    pub archor_intervals: i64,
    pub filename: Option<Varchar<1024>>,
    pub content_type: Varchar<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineKFileRsp {
    pub true_sid: Varchar<100>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineKFileReq {
    pub sid: Option<String>,
    pub meta_id: Option<Varchar<100>>,
    pub with_omit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineKFileRsp {
    pub res: Vec<InlineKFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryKFileReq {
    pub meta_id: Varchar<100>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryKFileMetaRsp {
    pub meta: Option<KFileMeta>,
}

#[test]
fn tst() {
    let s = "{\"res\":{\"tid\":1749701191901010,\"rid\":\"d0d48143-f149-40eb-bb65-739559a1e2d8\",\"kspace\":\"public\",\"archor\":false,\"name\":\"1749701191901011\",\"content\":\"\",\"content_type\":\"excalidraw-v1\"},\"archor_intervals\":3600}";
    let c: Result<InsertInlineKFileReq, serde_json::Error> = serde_json::from_str(s);
    c.unwrap();
}
