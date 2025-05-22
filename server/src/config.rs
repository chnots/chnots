use serde::Deserialize;

use crate::{
    mapper::{dump::filedump::FileBackupConfig, MapperConfig},
    server::ServerConfig,
};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AttachmentConfig {
    pub(crate) base_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    pub(crate) server: Option<ServerConfig>,
    pub(crate) mapper: MapperConfig,
    pub(crate) file_backup: Option<FileBackupConfig>,
    pub(crate) attachment: AttachmentConfig,
}

pub(crate) mod tests {
    #[test]
    fn test_db_deserialize() {
        let toml_str = r#"
        [db_config]
        type = "sqlite"
        filepath = "/home/123"
    "#;

        let config: super::Config = toml::from_str(toml_str).unwrap();
        println!("{:?}", config);
    }
}
