use chin_tools::AResult;
use std::{ops::Deref, sync::Arc};

use crate::{
    config::Config,
    mapper::{ChnotMapper, MapperType},
    model::dto::{
        chnot::{Chnot, ChnotQueryReq, ChnotQueryRsp},
        KReq,
    },
};

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

impl AppState {
    pub(crate) async fn chnot_query(
        &self,
        req: KReq<ChnotQueryReq>,
    ) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        self.mapper.chnot_query(req).await
    }
}
