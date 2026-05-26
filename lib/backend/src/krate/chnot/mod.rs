//! Chnot module — note metadata, threading, search, and ordering.
//!
//! Core domain managing `ChnotMeta` (note metadata) and `ChnotThreadOrder`
//! (thread relationships). All other content modules (mdwt, graph, ktab,
//! kfile, llmchat) are stored as different `ChnotKind` variants.

pub(crate) mod controller;
pub(crate) mod db;
pub(crate) mod dto;
pub(crate) mod mapper;
pub(crate) mod po;
pub(crate) mod sync;

pub(crate) use dto::*;
pub(crate) use po::*;
