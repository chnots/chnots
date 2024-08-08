use app::AppState;
use arguments::Arguments;
use chin_tools::wrapper::anyhow::AResult;
use clap::Parser;
use config::Config;
use mapper::{Db, MapperType, TableFounder};
use model::v1::domains::Domains;
use tracing::{info, Level};

pub mod app;
pub mod arguments;
pub mod config;
pub mod controller;
pub mod mapper;
pub mod model;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    let args = Arguments::parse();

    let config_file = tokio::fs::read_to_string(args.config.as_str()).await;
    match config_file {
        Ok(cf) => {
            let config: Config = toml::from_str(cf.as_str())?;
            let mapper: AResult<MapperType> = config.mapper.clone().into();
            let mapper = mapper?;
            mapper.ensure_tables().await?;

            let state = AppState {
                config,
                mapper,
                domains: Domains::new(),
            };
            let state: app::ShareAppState = state.into();

            state.mapper.set_app_state(state.clone());

            controller::serve(state).await;
        }
        Err(err) => {
            info!("unable to read err, creating default config to it. {}", err);
        }
    }

    Ok(())
}
