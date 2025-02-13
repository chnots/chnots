use app::{AppState, ShareAppState};
use arguments::Arguments;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use clap::Parser;
use config::Config;
use mapper::{
    backup::{
        filedump::{BackupType, FileDumpWorker},
        TableRowCallbackEnum,
    },
    MapperType,
};
use server::controller;
use tracing::{info, Level};
use tracing_log::LogTracer;

pub(crate) mod app;
pub(crate) mod arguments;
pub(crate) mod config;
pub(crate) mod magics;
pub(crate) mod mapper;
pub(crate) mod model;
pub(crate) mod server;
pub(crate) mod toent;
pub(crate) mod util;

#[tokio::main]
async fn main() -> EResult {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time());

    #[cfg(debug_assertions)]
    let subscriber = subscriber.with_max_level(Level::DEBUG);

    let subscriber = subscriber.finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    LogTracer::init()?;

    let args = Arguments::parse();

    let config_file = tokio::fs::read_to_string(args.config.as_str()).await?;
    let config: Config = toml::from_str(config_file.as_str())?;
    let mapper = AResult::<MapperType>::from(config.mapper.clone().into())?;
    mapper.ensure_tables().await?;
    let state = AppState {
        config: config.clone(),
        mapper,
    };
    let state: ShareAppState = state.into();
    {
        let state = state.clone();
        std::thread::spawn(|| {
            futures::executor::block_on(async move {
                let worker = FileDumpWorker::new(&state, "chnots", BackupType::All)
                    .await
                    .unwrap();
                info!("Begin to backup.");
                state
                    .mapper
                    .dump_and_backup(TableRowCallbackEnum::File(worker))
                    .await
                    .unwrap();
                info!("Finished to backup.");
            });
        });
    }

    controller::serve(state).await?;

    Ok(())
}
