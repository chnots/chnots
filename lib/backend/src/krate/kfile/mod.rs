//! KFile module — file upload, download, and inline storage.
//!
//! Manages binary file assets with inline (base64) and chunk upload support.
//! Used by graph (excalidraw/mind-elixir) for embedded images.

pub(crate) mod backup;
pub(crate) mod controller;
pub(crate) mod db;
pub(crate) mod dto;
pub(crate) mod mapper;
pub(crate) mod po;
pub(crate) mod sync;
mod transfer;

pub(crate) use dto::*;
pub(crate) use po::*;
