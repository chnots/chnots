use chin_tools::{AResult, EResult};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait MdwtMapper {
    async fn mdwt_commit(&self, req: KReq<MdwtCommitReq>) -> AResult<MdwtCommitRsp>;

    async fn mdwt_tag_refresh(&self) -> EResult;
    async fn mdwt_tag_list(&self, req: KReq<MdwtTagListReq>) -> AResult<MdwtTagListRsp<MdwtTag>>;
    async fn mdwt_tag_name_list(
        &self,
        req: KReq<MdwtTagListReq>,
    ) -> AResult<MdwtTagListRsp<String>>;

    async fn ensure_table_mdwt(&self) -> EResult;

    async fn mdwt_list(&self, req: KReq<MdwtRecordsReq>) -> AResult<MdwtRecordsRsp>;
}

impl MdwtMapper for MapperType {
    async fn mdwt_commit(&self, req: KReq<MdwtCommitReq>) -> AResult<MdwtCommitRsp> {
        expand_mt_branch!(self.mdwt_commit(req))
    }

    async fn ensure_table_mdwt(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_mdwt())
    }

    async fn mdwt_tag_list(&self, req: KReq<MdwtTagListReq>) -> AResult<MdwtTagListRsp<MdwtTag>> {
        expand_mt_branch!(self.mdwt_tag_list(req))
    }

    async fn mdwt_tag_name_list(
        &self,
        req: KReq<MdwtTagListReq>,
    ) -> AResult<MdwtTagListRsp<String>> {
        expand_mt_branch!(self.mdwt_tag_name_list(req))
    }

    async fn mdwt_tag_refresh(&self) -> EResult {
        expand_mt_branch!(self.mdwt_tag_refresh())
    }

    async fn mdwt_list(&self, req: KReq<MdwtRecordsReq>) -> AResult<MdwtRecordsRsp> {
        expand_mt_branch!(self.mdwt_list(req))
    }
}
