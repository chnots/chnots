//! KKV module — key-value store for metadata.
//!
//! Simple key-value storage for application metadata (client ID, DB version, etc.).
//! Leaf module with no krate dependencies.

pub(crate) mod backup;
pub(crate) mod controller;
pub(crate) mod db;
pub(crate) mod dto;
pub(crate) mod mapper;
pub(crate) mod po;
pub(crate) mod sync;

pub(crate) use dto::*;
pub(crate) use po::*;
