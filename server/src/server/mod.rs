pub mod controller;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub tls_key: String,
    pub tls_cert: String,
}
