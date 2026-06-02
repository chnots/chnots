use chin_tools::{AResult, EResult};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait KTabMapper {
    async fn ktab_meta_commit(&self, req: KReq<KTabMetaCommitReq>) -> AResult<KTabMetaCommitRsp>;

    async fn ktab_cell_commit(&self, req: KReq<KTabCellCommitReq>) -> AResult<KTabCellCommitRsp>;

    async fn ktab_meta_fetch(&self, req: KReq<KTabMetaFetchReq>) -> AResult<KTabMetaFetchRsp>;

    async fn ktab_cell_list(&self, req: KReq<KTabCellListReq>) -> AResult<KTabCellListRsp>;

    async fn ktab_row_delete(&self, req: KReq<KTabRowDeleteReq>) -> AResult<KTabRowDeleteRsp>;

    async fn ensure_ktab_tables(&self) -> EResult;
}

impl KTabMapper for MapperType {
    async fn ktab_meta_commit(&self, req: KReq<KTabMetaCommitReq>) -> AResult<KTabMetaCommitRsp> {
        expand_mt_branch!(self.ktab_meta_commit(req))
    }

    async fn ktab_cell_commit(&self, req: KReq<KTabCellCommitReq>) -> AResult<KTabCellCommitRsp> {
        expand_mt_branch!(self.ktab_cell_commit(req))
    }

    async fn ktab_meta_fetch(&self, req: KReq<KTabMetaFetchReq>) -> AResult<KTabMetaFetchRsp> {
        expand_mt_branch!(self.ktab_meta_fetch(req))
    }

    async fn ktab_cell_list(&self, req: KReq<KTabCellListReq>) -> AResult<KTabCellListRsp> {
        expand_mt_branch!(self.ktab_cell_list(req))
    }

    async fn ktab_row_delete(&self, req: KReq<KTabRowDeleteReq>) -> AResult<KTabRowDeleteRsp> {
        expand_mt_branch!(self.ktab_row_delete(req))
    }

    async fn ensure_ktab_tables(&self) -> EResult {
        expand_mt_branch!(self.ensure_ktab_tables())
    }
}
