use app::{AppState, ShareAppState};
use chin_tools::{AResult, EResult};
use config::Config;
use log::info;
use mapper::MapperType;

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
    info!("config {config:?}");

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

            if let Err(err) = state.dump_to_files(StartType::All).await {
                log::error!("unable to backup to files {err}")
            }

            log::info!("begin to sync via network {}", state.instance_id.as_str());
            if let Err(err) = state.sync_to_all_endpoints().await {
                log::error!("unable to backup via networks {err}, {}", err.backtrace())
            }
        });
    }

    controller::serve(state).await?;

    Ok(())
}
