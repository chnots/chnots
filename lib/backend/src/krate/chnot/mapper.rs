use chin_sql::str_type::Varchar;
use chin_tools::{AResult, EResult};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait ChnotMapper {
    async fn chnot_overwrite_mdwt(
        &self,
        req: KReq<ChnotOverwriteMdwtReq>,
    ) -> AResult<ChnotOverwriteMdwtRsp>;
    async fn chnot_overwrite_metas(
        &self,
        req: KReq<ChnotOverwriteMetaReq>,
    ) -> AResult<ChnotOverwriteMetaRsp>;
    async fn chnot_thread_query(
        &self,
        req: KReq<ChnotThreadQueryReq>,
    ) -> AResult<ChnotThreadQueryRsp>;
    async fn chnot_overwrite_thread_meta(
        &self,
        req: KReq<ChnotOverwriteThreadMetaReq>,
    ) -> AResult<ChnotOverwriteThreadMetaRsp>;

    async fn chnot_tag_update_all(&self, kspace: Varchar<40>) -> EResult;
    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
    ) -> AResult<ChnotThreadTagQueryRsp<ChnotThreadTag>>;
    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
    ) -> AResult<ChnotThreadTagQueryRsp<String>>;

    async fn chnot_thread_meta(&self, req: KReq<ChnotThreadMetaReq>)
    -> AResult<ChnotThreadMetaRsp>;
    async fn mdwt_blocks(&self, req: KReq<MdwtRecordsReq>) -> AResult<MdwtRecordsRsp>;

    async fn ensure_table_chnot(&self) -> EResult;
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite_mdwt(
        &self,
        req: KReq<ChnotOverwriteMdwtReq>,
    ) -> AResult<ChnotOverwriteMdwtRsp> {
        expand_mt_branch!(self.chnot_overwrite_mdwt(req))
    }

    async fn chnot_thread_query(
        &self,
        req: KReq<ChnotThreadQueryReq>,
    ) -> AResult<ChnotThreadQueryRsp> {
        expand_mt_branch!(self.chnot_thread_query(req))
    }

    async fn chnot_overwrite_thread_meta(
        &self,
        req: KReq<ChnotOverwriteThreadMetaReq>,
    ) -> AResult<ChnotOverwriteThreadMetaRsp> {
        expand_mt_branch!(self.chnot_overwrite_thread_meta(req))
    }

    async fn ensure_table_chnot(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot())
    }

    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
    ) -> AResult<ChnotThreadTagQueryRsp<ChnotThreadTag>> {
        expand_mt_branch!(self.chnot_tag_query(req))
    }

    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
    ) -> AResult<ChnotThreadTagQueryRsp<String>> {
        expand_mt_branch!(self.chnot_tag_names(req))
    }

    async fn chnot_tag_update_all(&self, kspace: Varchar<40>) -> EResult {
        expand_mt_branch!(self.chnot_tag_update_all(kspace))
    }

    async fn chnot_thread_meta(
        &self,
        req: KReq<ChnotThreadMetaReq>,
    ) -> AResult<ChnotThreadMetaRsp> {
        expand_mt_branch!(self.chnot_thread_meta(req))
    }

    async fn mdwt_blocks(&self, req: KReq<MdwtRecordsReq>) -> AResult<MdwtRecordsRsp> {
        expand_mt_branch!(self.mdwt_blocks(req))
    }

    async fn chnot_overwrite_metas(
        &self,
        req: KReq<ChnotOverwriteMetaReq>,
    ) -> AResult<ChnotOverwriteMetaRsp> {
        expand_mt_branch!(self.chnot_overwrite_metas(req))
    }
}
