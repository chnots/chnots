use chin_tools::AResult;
use std::{ops::Deref, sync::Arc};

use crate::{config::Config, mapper::MapperType, model::dto::KReq};

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

impl Into<ShareAppState> for AppState {
    fn into(self) -> ShareAppState {
        ShareAppState(Arc::new(self))
    }
}
