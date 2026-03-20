use std::{ops::Deref, sync::Arc};

use chin_tools::SharedStr;
use tokio::sync::RwLock;

use crate::{config::Config, krate::toent::cache::ToentCache, mapper::MapperType};

pub struct AppState {
    pub(crate) mapper: MapperType,
    pub(crate) config: Config,
    pub instance_id: SharedStr,
    pub(crate) toent_cache: RwLock<ToentCache>,
}

impl Deref for AppState {
    type Target = MapperType;

    fn deref(&self) -> &Self::Target {
        &self.mapper
    }
}

#[derive(Clone)]
pub struct ShareAppState(Arc<AppState>);

impl Deref for ShareAppState {
    type Target = AppState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<AppState> for ShareAppState {
    fn from(val: AppState) -> Self {
        ShareAppState(Arc::new(val))
    }
}
