use chin_tools::wrapper::anyhow::AResult;

use crate::{
    mapper::ChnotMapper,
    model::v1::dto::{
        ChnotDeletionReq, ChnotDeletionRsp, ChnotInsertionReq, ChnotInsertionRsp, ChnotQueryReq,
        ChnotQueryRsp, ReqWrapper,
    },
};

use super::AppState;

impl AppState {
    pub async fn chnot_overwrite(
        &self,
        req: ReqWrapper<ChnotInsertionReq>,
    ) -> AResult<ChnotInsertionRsp> {
        self.mapper.chnot_overwrite(req).await
    }

    pub async fn chnot_deletion(
        &self,
        req: ReqWrapper<ChnotDeletionReq>,
    ) -> AResult<ChnotDeletionRsp> {
        self.mapper.chnot_delete(req).await
    }

    pub async fn chnot_query(&self, req: ReqWrapper<ChnotQueryReq>) -> AResult<ChnotQueryRsp> {
        self.mapper.chnot_query(req).await
    }
}
