//! Sync module — file backup and network synchronization.
//!
//! Orchestrates backup/restore of all krate modules to local files
//! and network endpoints. Coordinates across all domain modules.

pub(crate) mod controller;
pub(crate) mod db;
pub mod dto;
pub(crate) mod filedumper;
pub(crate) mod mapper;
pub(crate) mod networksync;
pub(crate) mod po;
