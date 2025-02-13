pub mod chnot;
pub mod kv;
pub mod llmchat;

/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use std::{fmt::Debug, ops::Deref};

use axum::{extract::Multipart, http::HeaderMap};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    db::resource::{InlineResource, Resource},
    shared_str::SharedStr,
};

#[derive(Debug, Clone, Serialize)]
pub struct KReq<E: Debug + Clone + DeserializeOwned> {
    pub body: E,
    pub namespace: String,
}

pub fn read_namespace_from_header(headers: &HeaderMap) -> String {
    headers
        .get("K-namespace")
        .and_then(|v| v.to_str().ok().map(|e| e.to_string()))
        .unwrap()
}

pub fn kreq<E: Debug + Clone + DeserializeOwned>(headers: HeaderMap, body: E) -> KReq<E> {
    KReq {
        body,
        namespace: read_namespace_from_header(&headers),
    }
}

impl<E: Debug + Clone + DeserializeOwned> Deref for KReq<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineResourceReq {
    pub res: InlineResource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertInlineResourceRsp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineResourceReq {
    pub id: Option<SharedStr>,
    pub content_type: Option<SharedStr>,
    pub name_like: Option<SharedStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryInlineResourceRsp {
    pub res: Vec<InlineResource>,
}
