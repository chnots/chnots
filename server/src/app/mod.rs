pub mod v1;

use std::{ops::Deref, sync::Arc};

use crate::{config::Config, mapper::MapperType, model::v1::domains::Domains};

pub struct AppState {
    pub mapper: MapperType,
    pub config: Config,
    pub domains: Domains,
}

impl AppState {
    pub fn new(mapper: MapperType, config: Config, domains: Domains) -> Self {
        let state = AppState {
            mapper,
            config,
            domains,
        };

        state
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

impl Into<ShareAppState> for AppState {
    fn into(self) -> ShareAppState {
        ShareAppState(Arc::new(self))
    }
}
