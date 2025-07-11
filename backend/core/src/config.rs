use serde::Deserialize;

use crate::krate::sync::filedumper::FileBackupConfig;
pub use crate::mapper::{MapperConfig};
pub use crate::mapper::db::sqlite::SqliteConfig;

#[cfg(feature = "tls")]    
#[derive(Debug, Clone, Deserialize)]
pub struct TlsConfig {
    pub tls_key: String,
    pub tls_cert: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    #[cfg(feature = "tls")]    
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttachmentConfig {
    pub base_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub mapper: MapperConfig,
    pub file_backup: Option<FileBackupConfig>,
    pub attachment: AttachmentConfig,
}
