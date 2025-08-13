use std::ops::Deref;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::krate::sync::filedumper::FileBackupConfig;
pub use crate::mapper::MapperConfig;
pub use crate::mapper::db::sqlite::SqliteConfig;

#[derive(Debug, Clone)]
pub struct ShellExpandPath {
    #[allow(dead_code)]
    original: String,
    parsed: String,
}

impl Deref for ShellExpandPath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.parsed
    }
}

impl From<ShellExpandPath> for PathBuf {
    fn from(val: ShellExpandPath) -> Self {
        val.parsed.into()
    }
}

impl<'de> Deserialize<'de> for ShellExpandPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ori = String::deserialize(deserializer)?;
        Ok(Self::from(ori))
    }
}

impl From<String> for ShellExpandPath {
    fn from(value: String) -> Self {
        let mut original = value;
        if original.starts_with("./") {
            original = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string()
                + &original[1..];
        }
        let parsed: std::borrow::Cow<'_, str> = shellexpand::tilde(original.as_str());
        Self {
            parsed: parsed.to_string(),
            original,
        }
    }
}

impl AsRef<Path> for ShellExpandPath {
    fn as_ref(&self) -> &Path {
        self.parsed.as_ref()
    }
}

#[cfg(feature = "tls")]
#[derive(Debug, Clone, Deserialize)]
pub struct TlsConfig {
    pub tls_key: ShellExpandPath,
    pub tls_cert: ShellExpandPath,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    #[cfg(feature = "tls")]
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttachmentConfig {
    pub base_dir: ShellExpandPath,
}

impl AttachmentConfig {
    pub(crate) fn get_sid_path<S: AsRef<str>>(&self, sid: S) -> PathBuf {
        let trimed = sid.as_ref().replace("-", "");
        let trimed = trimed.trim();
        if trimed.len() < 5 {
            return std::path::Path::new(&self.base_dir.as_str()).join(trimed);
        }
        std::path::Path::new(&self.base_dir.as_str())
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
