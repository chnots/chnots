//! KTab module — table/spreadsheet functionality.
//!
//! Dynamic tables with typed columns (string, decimal, date) and
//! cell-level CRUD operations.

pub(crate) mod backup;
pub(crate) mod controller;
pub(crate) mod db;
pub(crate) mod dto;
pub(crate) mod mapper;
pub(crate) mod po;
pub mod sync;

pub(crate) use dto::*;
pub(crate) use po::*;
