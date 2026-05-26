use app::{AppState, ShareAppState};
use chin_tools::{AResult, EResult, utils::id_util::generate_uuid};
use config::Config;
use log::info;
use mapper::MapperType;

use crate::{
    krate::{sync::filedumper::StartType, toent::cache::ToentCache},
    magics::CLIENT_ID_KEY,
};

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
    let instance_id = match mapper.fetch_chnot_meta_value(CLIENT_ID_KEY).await? {
        Some(instance_id) => instance_id,
        None => {
            let instance_id = generate_uuid();
            mapper
                .commit_chnot_meta_value(
                    CLIENT_ID_KEY,
                    instance_id.clone(),
                    chin_sql::OnConflict::Default,
                )
                .await?;
            instance_id
        }
    };
    let state = AppState {
        config: config.clone(),
        mapper,
        instance_id: instance_id.into(),
        toent_cache: Default::default(),
    };

    let state: ShareAppState = state.into();

    if let Err(err) = ToentCache::refresh(&state).await {
        log::warn!("unable to warm up toent daily cache on startup: {err}");
    }

    {
        let state = state.clone();
        tokio::spawn(async move {
            info!("Begin to backup.");

            if let Err(err) = state.dump_to_files(StartType::All).await {
                log::error!("unable to backup to files {err}, {}", err.backtrace())
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
