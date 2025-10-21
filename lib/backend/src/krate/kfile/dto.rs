use super::*;
use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use chin_sql::{str_type::Varchar, time_type::TID};
use serde::{Deserialize, Serialize};

#[derive(TryFromMultipart)]
pub struct KfileAssetChunkUploadReq {
    pub upload_id: String,
    pub meta_id: String,
    pub otid: i64,
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
pub struct KfileInlineUploadReq {
    pub meta_id: Varchar<100>,
    pub otid: TID,
    pub res: InlineKFile,
    /// archor interval second.
    pub archor_intervals: i64,
    pub filename: Option<Varchar<1024>>,
    pub content_type: Varchar<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineUploadRsp {
    pub true_sid: Varchar<100>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineDownloadReq {
    pub sid: Option<Varchar<100>>,
    pub meta_id: Option<Varchar<100>>,
    pub with_omit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineDownloadRsp {
    pub res: Vec<InlineKFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileMetaFetchReq {
    pub meta_id: Varchar<100>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileMetaFetchRsp {
    pub meta: Option<KFileMeta>,
}

pub const KFILE_INLINE_UPLOAD_DIRECTLY: &str = "/api/v1/kfile-inline-upload-directly";
pub const KFILE_INLINE_DOWNLOAD_BY_SID: &str = "/api/v1/kfile-inline-download-by-sid";
pub const KFILE_ASSET_UPLOAD_BY_SID: &str = "/api/v1/kfile-asset-upload-by-sid";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineUploadDirectlyReq {
    pub file: InlineKFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineUploadDirectlyRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineDownloadBySidReq {
    pub sid: Varchar<100>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileInlineDownloadBySidRsp {
    pub file: Option<InlineKFile>,
}

#[test]
fn tst() {
    let s = "{\"res\":{\"tid\":1749701191901010,\"rid\":\"d0d48143-f149-40eb-bb65-739559a1e2d8\",\"kspace\":\"public\",\"archor\":false,\"name\":\"1749701191901011\",\"content\":\"\",\"content_type\":\"excalidraw-v1\"},\"archor_intervals\":3600}";
    let c: Result<KfileInlineUploadReq, serde_json::Error> = serde_json::from_str(s);
    c.unwrap();
}
