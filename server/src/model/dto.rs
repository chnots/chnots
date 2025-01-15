/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use std::{fmt::Debug, ops::Deref};

use axum::{extract::Multipart, http::HeaderMap};

use serde::{Deserialize, Serialize};


use crate::toent::PossibleToent;

use super::db::{
    chnot::{ChnotKind, ChnotMetadata, ChnotRecord},
    resource::Resource,
};

#[derive(Debug, Clone, Serialize)]
pub struct KReq<E: Debug + Clone + Serialize> {
    pub body: E,
    pub namespace: String,
}

pub fn read_namespace_from_header(headers: &HeaderMap) -> String {
    headers
        .get("K-namespace")
        .and_then(|v| v.to_str().ok().map(|e| e.to_string())).unwrap()
}

pub fn kreq<E: Debug + Clone + Serialize>(headers: HeaderMap, body: E) -> KReq<E> {
    KReq {
        body,
        namespace: read_namespace_from_header(&headers),
    }
}

impl<E: Debug + Clone + Serialize> Deref for KReq<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chnot {
    pub record: ChnotRecord,
    pub meta: ChnotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateReq {
    pub chnot_meta_id: String,

    pub update_time: bool,

    pub pinned: Option<bool>,
    pub archive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotUpdateRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteReq {
    pub chnot: ChnotRecord,
    pub kind: ChnotKind,
}

impl Deref for ChnotOverwriteReq {
    type Target = ChnotRecord;

    fn deref(&self) -> &Self::Target {
        &self.chnot
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotOverwriteRsp {
    pub chnot: Chnot
}

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
    pub with_deleted: Option<bool>,
    pub with_omitted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChnotQueryRsp<T> {
    pub data: T,
    pub start_index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceQueryReq {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceQueryRsp {}

pub type ResourceUploadReq = Multipart;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUploadRsp {
    pub(crate) resources: Vec<Resource>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToentGuessReq {
    pub input: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ToentGuessRsp {
    pub toents: Vec<PossibleToent>
}