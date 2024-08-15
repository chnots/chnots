use serde::Deserialize;

use crate::{
    mapper::{backup::filebackup::FileBackupConfig, MapperConfig},
    server::ServerConfig,
};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub mapper: MapperConfig,
    pub backup: Option<FileBackupConfig>,
}

pub mod tests {
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
