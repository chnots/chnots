use app::{AppState, ShareAppState};
use chin_tools::{AResult, EResult};
use config::Config;
use log::info;
use mapper::{
    MapperType,
    dump::{
        RecordCallbackType,
        filedump::{BackupType, FileDumpWorker},
    },
};

#[cfg(not(feature = "tauri"))]
use log::Level;
#[cfg(not(feature = "tauri"))]
use tracing_log::LogTracer;

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
    let state = AppState {
        config: config.clone(),
        mapper,
    };
    let state: ShareAppState = state.into();
    {
        let state = state.clone();
        std::thread::spawn(|| {
            futures::executor::block_on(async move {
                let Ok(worker) = FileDumpWorker::new(&state, "chnots", BackupType::All).await
                else {
                    return;
                };
                info!("Begin to backup.");
            });
        });
    }

    controller::serve(state).await?;

    Ok(())
}
