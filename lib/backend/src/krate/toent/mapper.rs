use anyhow::bail;
use chin_tools::{AResult, EResult};

use crate::{
    MapperType, expand_mt_branch,
    krate::toent::{
        ToentInstCountReq, ToentInstCountRsp, ToentSearchReq, ToentSearchRsp,
        ToentTodoStateCommitReq, ToentTodoStateCommitRsp,
        po::{ToentDefi, ToentInst},
    },
    mapper::db::KDbTx,
    model::dto::KReq,
};

pub(crate) trait ToentMapper {
    async fn toent_inst_count(&self, _req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp> {
        bail!("toent_inst_count is unsupported")
    }

    async fn toent_search(&self, _req: KReq<ToentSearchReq>) -> AResult<ToentSearchRsp> {
        bail!("toent_search is unsupported")
    }

    async fn toent_todo_state_commit(
        &self,
        _req: KReq<ToentTodoStateCommitReq>,
    ) -> AResult<ToentTodoStateCommitRsp> {
        bail!("toent_todo_state_commit is unsupported")
    }
}

impl ToentMapper for MapperType {
    async fn toent_inst_count(&self, req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp> {
        expand_mt_branch!(self.toent_inst_count(req))
    }

    async fn toent_search(&self, req: KReq<ToentSearchReq>) -> AResult<ToentSearchRsp> {
        expand_mt_branch!(self.toent_search(req))
    }

    async fn toent_todo_state_commit(
        &self,
        req: KReq<ToentTodoStateCommitReq>,
    ) -> AResult<ToentTodoStateCommitRsp> {
        expand_mt_branch!(self.toent_todo_state_commit(req))
    }
}
