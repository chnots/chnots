use chin_tools::{AResult, EResult};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait ChnotMapper {
    async fn chnot_meta_commit(&self, req: KReq<ChnotMetaCommitReq>)
    -> AResult<ChnotMetaCommitRsp>;
    async fn chnot_overwrite_thread_orders(
        &self,
        req: KReq<ChnotThreadOrderCommitReq>,
    ) -> AResult<ChnotThreadOrderCommitRsp>;

    async fn chnot_thread_list(&self, req: KReq<ChnotThreadListReq>)
    -> AResult<ChnotThreadListRsp>;
    async fn chnot_thread_meta_commit(
        &self,
        req: KReq<ChnotThreadMetaFetchCommitReq>,
    ) -> AResult<ChnotThreadMetaFetchCommitRsp>;

    async fn chnot_thread_meta_fetch(
        &self,
        req: KReq<ChnotThreadMetaFetchReq>,
    ) -> AResult<ChnotThreadMetaFetchRsp>;

    async fn ensure_table_chnot(&self) -> EResult;
}

impl ChnotMapper for MapperType {
    async fn chnot_thread_list(
        &self,
        req: KReq<ChnotThreadListReq>,
    ) -> AResult<ChnotThreadListRsp> {
        expand_mt_branch!(self.chnot_thread_list(req))
    }

    async fn chnot_thread_meta_commit(
        &self,
        req: KReq<ChnotThreadMetaFetchCommitReq>,
    ) -> AResult<ChnotThreadMetaFetchCommitRsp> {
        expand_mt_branch!(self.chnot_thread_meta_commit(req))
    }

    async fn ensure_table_chnot(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot())
    }

    async fn chnot_thread_meta_fetch(
        &self,
        req: KReq<ChnotThreadMetaFetchReq>,
    ) -> AResult<ChnotThreadMetaFetchRsp> {
        expand_mt_branch!(self.chnot_thread_meta_fetch(req))
    }

    async fn chnot_overwrite_thread_orders(
        &self,
        req: KReq<ChnotThreadOrderCommitReq>,
    ) -> AResult<ChnotThreadOrderCommitRsp> {
        expand_mt_branch!(self.chnot_overwrite_thread_orders(req))
    }

    async fn chnot_meta_commit(
        &self,
        req: KReq<ChnotMetaCommitReq>,
    ) -> AResult<ChnotMetaCommitRsp> {
        expand_mt_branch!(self.chnot_meta_commit(req))
    }
}
