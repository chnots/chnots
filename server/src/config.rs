use serde::Deserialize;

use crate::mapper::MapperConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: Option<Server>,
    pub mapper: MapperConfig,
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
