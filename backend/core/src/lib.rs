use app::{AppState, ShareAppState};
use chin_tools::{AResult, EResult};
use config::Config;
use log::info;
use mapper::MapperType;

#[cfg(not(feature = "tauri"))]
use log::Level;
#[cfg(not(feature = "tauri"))]
use tracing_log::LogTracer;

use crate::krate::sync::filedumper::StartType;

pub(crate) mod app;
pub mod config;
pub(crate) mod controller;
pub(crate) mod krate;
pub(crate) mod magics;
pub(crate) mod mapper;
pub(crate) mod model;
pub(crate) mod util;

pub async fn run(config: Config) -> EResult {
    #[cfg(not(feature = "tauri"))]
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time());
    #[cfg(not(feature = "tauri"))]
    LogTracer::init()?;

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "tauri"))]
    let subscriber = subscriber.with_max_level(tracing::Level::DEBUG);

    #[cfg(not(feature = "tauri"))]
    let subscriber = subscriber.finish();

    #[cfg(not(feature = "tauri"))]
    tracing::subscriber::set_global_default(subscriber)?;

    let mapper = AResult::<MapperType>::from(config.mapper.clone().try_into())?;
    mapper.ensure_tables().await?;
    let instance_id = mapper.get_instance_id().await?;
    let state = AppState {
        config: config.clone(),
        mapper,
        instance_id: instance_id.into(),
    };
    let state: ShareAppState = state.into();
    {
        let state = state.clone();
        tokio::spawn(async move {
            info!("Begin to backup.");

            if let Err(err) = state.init_instance_id().await {
                info!("unable to create instace_id {err}");
            }

            if let Err(err) = state.dump_all_to_files(StartType::Increase).await {
                log::error!("unable to backup to files {err}")
            }

            log::info!("begin to sync via network");
            if let Err(err) = state.sync_via_network().await {
                log::error!("unable to backup via networks {err}")
            }
        });
    }

    controller::serve(state).await?;

    Ok(())
}
