use app::{AppState, ShareAppState};
use chin_tools::{AResult, EResult};
use config::Config;
use mapper::{
    MapperType,
    dump::{
        RecordCallbackType,
        filedump::{BackupType, FileDumpWorker},
    },
};
use tracing::{Level, info};
use tracing_log::LogTracer;

pub(crate) mod app;
pub mod config;
pub(crate) mod controller;
pub(crate) mod krate;
pub(crate) mod magics;
pub(crate) mod mapper;
pub(crate) mod model;
pub(crate) mod util;

pub async fn start(config: Config) -> EResult {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::time());
    LogTracer::init()?;

    #[cfg(debug_assertions)]
    let subscriber = subscriber.with_max_level(Level::DEBUG);

    let subscriber = subscriber.finish();

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
                let worker = FileDumpWorker::new(&state, "chnots", BackupType::All)
                    .await
                    .unwrap();
                info!("Begin to backup.");
                state
                    .mapper
                    .dump_and_callback(&RecordCallbackType::File(worker))
                    .await
                    .unwrap();
                info!("Finished to backup.");
            });
        });
    }

    controller::serve(state).await?;

    Ok(())
}
