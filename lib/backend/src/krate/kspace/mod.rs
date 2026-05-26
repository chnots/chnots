//! KSpace module — workspace management.
//!
//! Multi-workspace support for isolating notes and data.
//! Leaf module with no krate dependencies.

pub(crate) mod controller;
pub(crate) mod db;
pub(crate) mod dto;
pub(crate) mod mapper;
pub(crate) mod po;
pub(crate) mod sync;

pub(crate) use po::*;
