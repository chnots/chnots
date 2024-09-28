use std::{ops::Deref, sync::Arc};

use chin_tools::wrapper::anyhow::AResult;

use crate::{
    config::Config,
    mapper::{ChnotMapper, MapperType},
    model::v1::{
        convert::construct_rings,
        domains::Domains,
        dto::{ChnotQueryReq, ChnotQueryRsp, ChnotRing, KReq},
    },
};

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

impl AppState {
    pub async fn chnot_query(
        &self,
        req: KReq<ChnotQueryReq>,
    ) -> AResult<ChnotQueryRsp<Vec<ChnotRing>>> {
        let res = self.mapper.chnot_query(req).await?;

        Ok(ChnotQueryRsp {
            data: construct_rings(res.data, false),
            start_index: res.start_index,
        })
    }
}
