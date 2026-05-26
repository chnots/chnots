//! Mdwt module — markdown with time events (rich text editor backend).
//!
//! Handles markdown content storage, history, and tag-based search.
//! Integrates with `toent` for time event parsing in markdown content.

pub(crate) mod controller;
pub(crate) mod db;
pub(crate) mod dto;
pub(crate) mod mapper;
pub(crate) mod po;
pub(crate) mod sync;
// TODO remove this dead code
#[allow(dead_code)]
pub(crate) mod parser;

pub(crate) use dto::*;
pub(crate) use po::*;
