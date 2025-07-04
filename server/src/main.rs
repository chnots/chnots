use chnots_core::config::Config;
use clap::Parser;

use crate::arguments::Arguments;

pub(crate) mod arguments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    let config_file = tokio::fs::read_to_string(args.config.as_str()).await?;
    let config: Config = toml::from_str(config_file.as_str())?;

    chnots_core::run(config).await?;

    Ok(())
}
