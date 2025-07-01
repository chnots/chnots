/// DTO: Data Transfer Object
///
/// All dtos should be put into this file.
use std::{fmt::Debug, ops::Deref};

use axum::http::HeaderMap;

use chin_sql::str_type::Varchar;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, Serialize)]
pub(crate) struct KReq<E: Debug + Clone + DeserializeOwned> {
    pub(crate) body: E,
    pub(crate) kspace: Varchar<40>,
    pub(crate) mkspaces: Vec<Varchar<40>>,
}

impl<E: Debug + Clone + DeserializeOwned> KReq<E> {
    pub fn get_spaces(&self) -> Vec<Varchar<40>> {
        let mut c: Vec<Varchar<40>> = self
            .mkspaces
            .iter()
            .filter(|e| !e.as_str().is_empty())
            .map(|e| e.to_owned())
            .collect();
        c.push(self.kspace.clone());
        c
    }
}

pub(crate) fn read_kspace_from_header(headers: &HeaderMap) -> Varchar<40> {
    headers
        .get("K-kspace")
        .and_then(|v| v.to_str().ok().map(|e| e.to_string()))
        .unwrap()
        .try_into()
        .unwrap()
}

pub(crate) fn kreq<E: Debug + Clone + DeserializeOwned>(headers: HeaderMap, body: E) -> KReq<E> {
    let mkspaces: Vec<Varchar<40>> = headers
        .get("K-mkspaces")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .split(",")
        .map(|e| e.trim().to_owned())
        .filter(|s| !s.is_empty())
        .map(|c| c.try_into().unwrap())
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
