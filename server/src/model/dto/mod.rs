/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use std::{fmt::Debug, ops::Deref};

use axum::http::HeaderMap;

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct KReq<E: Debug + Clone + DeserializeOwned> {
    pub(crate) body: E,
    pub(crate) kspace: String,
    pub(crate) mkspaces: Vec<String>,
}

pub(crate) fn read_kspace_from_header(headers: &HeaderMap) -> String {
    headers
        .get("K-kspace")
        .and_then(|v| v.to_str().ok().map(|e| e.to_string()))
        .unwrap()
}

pub(crate) fn kreq<E: Debug + Clone + DeserializeOwned>(headers: HeaderMap, body: E) -> KReq<E> {
    let mkspaces: Vec<String> = headers
        .get("K-mkspaces")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .split(",")
        .into_iter()       
        .map(|e| e.trim().to_owned()) 
        .collect();

    KReq {
        body,
        kspace: read_kspace_from_header(&headers),
        mkspaces,
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
            kspace: self.kspace.clone(),
            mkspaces: self.mkspaces.clone(),
        }
    }
}
