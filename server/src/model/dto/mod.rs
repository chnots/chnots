/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use std::{fmt::Debug, ops::Deref};

use axum::http::HeaderMap;

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct KReq<E: Debug + Clone + DeserializeOwned> {
    pub(crate) body: E,
    pub(crate) workspace: String,
}

pub(crate) fn read_workspace_from_header(headers: &HeaderMap) -> String {
    headers
        .get("K-workspace")
        .and_then(|v| v.to_str().ok().map(|e| e.to_string()))
        .unwrap()
}

pub(crate) fn kreq<E: Debug + Clone + DeserializeOwned>(headers: HeaderMap, body: E) -> KReq<E> {
    KReq {
        body,
        workspace: read_workspace_from_header(&headers),
    }
}

impl<E: Debug + Clone + DeserializeOwned> Deref for KReq<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl<T> KReq<T>
where
    T: Debug + Clone + DeserializeOwned,
{
    pub(crate) fn frame<E>(&self, t: E) -> KReq<E>
    where
        E: Debug + Clone + DeserializeOwned,
    {
        KReq {
            body: t,
            workspace: self.workspace.clone(),
        }
    }
}
