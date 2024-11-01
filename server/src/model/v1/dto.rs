use std::{fmt::Debug, ops::Deref};

use axum::{extract::Multipart, http::HeaderMap};

use chrono::{DateTime, FixedOffset};
/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use serde::{Deserialize, Serialize};

use super::db::{
    chnot::{Chnot, ChnotType},
    resource::Resource,
};

#[derive(Debug, Clone, Serialize)]
pub struct KReq<E: Debug + Clone + Serialize> {
    pub body: E,
    pub domain: Option<String>,
}

pub fn kreq<E: Debug + Clone + Serialize>(headers: HeaderMap, body: E) -> KReq<E> {
    let domain: Option<String> = headers
        .get("K-Domain")
        .and_then(|v| v.to_str().ok().map(|e| e.to_string()));

    KReq { body, domain }
}

impl<E: Debug + Clone + Serialize> Deref for KReq<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateReq {
    pub chnot_id: String,

    pub update_time: bool,

    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotInsertionReq {
    pub chnot: Chnot,
    pub parent_id: Option<String>,
    pub prev_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotInsertionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionReq {
    pub chnot_id: String,
    /// logic or physical deletion
    pub logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotDeletionRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryReq {
    pub query: Option<String>,

    // Paging
    pub start_index: u64,
    pub page_size: u64,
}

pub struct ChnotWithRelation {
    pub chnot: Chnot,
    pub prev_id: Option<String>,
    pub parent_id: Option<String>,
}

impl Deref for ChnotWithRelation {
    type Target = Chnot;

    fn deref(&self) -> &Self::Target {
        &self.chnot
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedChnot {
    pub chnot: Chnot,
    pub children: Vec<NestedChnot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotRing {
    pub chnots: Vec<NestedChnot>,
    pub ring_id: String,
    pub r#type: ChnotType,
    pub init_time: DateTime<FixedOffset>,
    pub update_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp<T> {
    pub data: T,
    pub start_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainQueryReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainQueryRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotCommentAddReq {
    pub chnot_perm_id: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub insert_time: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotCommentAddRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotCommentDeleteReq {
    pub id: String,
    pub logic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotCommentDeleteRsp {}

// We do not allow user to update comment.
/* #[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotCommentUpdateReq {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotCommentUpdateRsp {} */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateListReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateListRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateOverwriteReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateOverwriteRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateDeleteReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatTemplateDeleteRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryListReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryListRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryDetailReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryDetailRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryAddReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatHistoryAddRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfigListReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfigListRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfigOverwriteReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfigOverwriteRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfigDeleteReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMChatConfigDeleteRsp {}

pub type ResourceUploadReq = Multipart;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUploadRsp {
    pub(crate) resources: Vec<Resource>,
}
