use chin_tools::wrapper::anyhow::{AResult, EResult};

use crate::{
    db::ChnotMapper,
    model::{
        chnot::Chnot,
        dto::{
            ChnotDeletionReq, ChnotDeletionRsp, ChnotInsertionReq, ChnotInsertionRsp,
            ChnotQueryReq, ChnotQueryRsp,
        },
    },
};

use super::AppState;

impl AppState {
    pub async fn chnot_overwrite(&self, chnot: ChnotInsertionReq) -> AResult<ChnotInsertionRsp> {
        self.mapper.chnot_overwrite(chnot).await
    }

    pub async fn chnot_deletion(&self, req: ChnotDeletionReq) -> AResult<ChnotDeletionRsp> {
        self.mapper.chnot_delete(req).await
    }

    pub async fn chnot_query(&self, req: ChnotQueryReq) -> AResult<ChnotQueryRsp> {
        self.mapper.chnot_query(req).await
    }
}
