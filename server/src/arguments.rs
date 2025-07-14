use clap::Parser;

#[derive(Parser, Default, Debug, Clone)]
#[command(author = "aeghn", version = "0.1", about = "chnots server")]
pub(crate) struct Arguments {
    #[clap(long, short, help = "Config file to read")]
    pub(crate) config: String,
}

pub mod tests {
    #[test]
    fn test_db_deserialize() {
        let toml_str = r#"
        [db_config]
        type = "sqlite"
        filepath = "/home/123"
    "#;

        let config: chnots_core::config::Config = toml::from_str(toml_str).unwrap();
        println!("{config:?}");
    }
}
