use chnots_core::config::Config;
use clap::Parser;

use crate::arguments::Arguments;
use tracing_log::LogTracer;
pub(crate) mod arguments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time());
    LogTracer::init()?;

    #[cfg(debug_assertions)]
    let subscriber = subscriber.with_max_level(tracing::Level::DEBUG);

    let subscriber = subscriber.finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let args = Arguments::parse();

    let config_file = tokio::fs::read_to_string(args.config.as_str()).await?;
    let config: Config = toml::from_str(config_file.as_str())?;

    chnots_core::run(config).await?;

    Ok(())
}
