pub mod v1;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{config::Config, db::MapperType};

pub struct AppState {
    pub mapper: MapperType,
    pub config: Config,
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
