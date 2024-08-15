pub mod controller;

use std::{ops::Deref, sync::Arc};

use serde::Deserialize;

use crate::{config::Config, mapper::MapperType, model::v1::domains::Domains};

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

