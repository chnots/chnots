pub(crate) mod controller;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ServerConfig {
    pub(crate) port: u16,
    pub(crate) tls_key: String,
    pub(crate) tls_cert: String,
}
