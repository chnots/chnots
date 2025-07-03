use serde::Deserialize;

use crate::mapper::{MapperConfig, dump::filedump::FileBackupConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub tls_key: String,
    pub tls_cert: String,
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
