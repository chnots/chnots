use chin_sql::str_type::Varchar;
use chin_tools::AResult;

use crate::{
    MapperType, expand_mt_branch,
    krate::toent::{
        ToentInstCommitReq, ToentInstCommitRsp, ToentInstCountReq, ToentInstCountRsp,
        ToentSearchReq, ToentSearchRsp,
    },
    model::dto::KReq,
};

pub(crate) trait ToentMapper {
    async fn toent_inst_count(&self, req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp>;
    async fn toent_search(
        &self,
        req: ToentSearchReq,
        spaces: Vec<Varchar<40>>,
    ) -> AResult<ToentSearchRsp>;
    async fn toent_inst_commit(&self, req: KReq<ToentInstCommitReq>)
    -> AResult<ToentInstCommitRsp>;
}

impl ToentMapper for MapperType {
    async fn toent_inst_count(&self, req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp> {
        expand_mt_branch!(self.toent_inst_count(req))
    }

    async fn toent_search(
        &self,
        req: ToentSearchReq,
        spaces: Vec<Varchar<40>>,
    ) -> AResult<ToentSearchRsp> {
        expand_mt_branch!(self.toent_search(req, spaces))
    }

    async fn toent_inst_commit(
        &self,
        req: KReq<ToentInstCommitReq>,
    ) -> AResult<ToentInstCommitRsp> {
        expand_mt_branch!(self.toent_inst_commit(req))
    }
}
