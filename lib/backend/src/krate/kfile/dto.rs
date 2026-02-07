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
    pub db_store: Option<bool>,
    pub binaryp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KFileUploadRsp {
    pub kfile: Option<KFileMeta>,
    pub finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKFileUploadReq {
    pub meta_id: Varchar<100>,
    pub otid: TID,
    pub res: InlineKFile,
    /// archor interval second.
    pub archor_intervals: i64,
    pub filename: Option<Varchar<1024>>,
    pub content_type: Varchar<200>,
    pub binaryp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKFileUploadRsp {
    pub true_sid: Varchar<100>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKFileDownloadReq {
    pub req_id: KfileMetaFetchReqId,
    pub with_omit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKFileDownloadRsp {
    pub meta: Option<KFileMeta>,
    pub file: Option<InlineKFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KfileMetaFetchReqId {
    Otid(TID),
    Id(Varchar<100>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileMetaFetchReq {
    pub req_id: KfileMetaFetchReqId,
    pub history_and_archor: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KfileMetaFetchRsp {
    pub meta: Option<KFileMeta>,
}

pub const KFILE_ASSET_UPLOAD_BY_SID: &str = "/api/v1/kfile-asset-upload-by-sid";

#[test]
fn tst() {
    let c = KfileMetaFetchReqId::Otid(100.try_into().unwrap());
    println!("{}", serde_json::to_string_pretty(&c).unwrap());
}
