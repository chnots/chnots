pub mod chnot;
pub mod resource;
pub mod llmchat;

/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use std::{fmt::Debug, ops::Deref};

use axum::{extract::Multipart, http::HeaderMap};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::db::resource::{InlineResource, Resource};

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

