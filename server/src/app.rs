use std::{ops::Deref, sync::Arc};

use crate::{config::Config, mapper::MapperType};

pub(crate) struct AppState {
    pub(crate) mapper: MapperType,
    pub(crate) config: Config,
}

#[derive(Clone)]
pub(crate) struct ShareAppState(Arc<AppState>);

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
