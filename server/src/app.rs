use std::{ops::Deref, sync::Arc};

use chin_tools::wrapper::anyhow::AResult;

use crate::{
    config::Config,
    mapper::{ChnotMapper, MapperType},
    model::dto::{Chnot, ChnotQueryReq, ChnotQueryRsp, KReq},
};

pub struct AppState {
    pub mapper: MapperType,
    pub config: Config,
}

impl AppState {
    pub fn new(mapper: MapperType, config: Config) -> Self {
        AppState { mapper, config }
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

impl AppState {
    pub async fn chnot_query(
        &self,
        req: KReq<ChnotQueryReq>,
    ) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        self.mapper.chnot_query(req).await
    }
}
