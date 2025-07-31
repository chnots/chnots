use std::path::PathBuf;

use serde::Deserialize;

use crate::krate::sync::filedumper::FileBackupConfig;
pub use crate::mapper::MapperConfig;
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

impl AttachmentConfig {
    pub(crate) fn get_sid_path<S: AsRef<str>>(&self, sid: S) -> PathBuf {
        let trimed = sid.as_ref().replace("-", "");
        let trimed = trimed.trim();
        if trimed.len() < 5 {
            return std::path::Path::new(&self.base_dir).join(trimed);
        }
        std::path::Path::new(&self.base_dir)
            .join(&trimed[0..2])
            .join(&trimed[2..4])
            .join(&trimed[4..])
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub mapper: MapperConfig,
    pub file_backup: Option<FileBackupConfig>,
    pub attachment: AttachmentConfig,
}
