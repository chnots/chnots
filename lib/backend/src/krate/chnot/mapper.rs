use chin_sql::str_type::Varchar;
use chin_tools::{AResult, EResult};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait ChnotMapper {
    async fn chnot_overwrite_record(
        &self,
        req: KReq<ChnotOverwriteRecordReq>,
    ) -> AResult<ChnotOverwriteRecordRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Chnot>>;
    async fn chnot_query_kind_rel(
        &self,
        req: KReq<ChnotKindRelQueryReq>,
    ) -> AResult<ChnotKindRelQueryRsp>;
    async fn chnot_overwrite_meta(
        &self,
        req: KReq<ChnotOverwriteMetaReq>,
    ) -> AResult<ChnotOverwriteMetaRsp>;

    async fn chnot_tag_update_all(&self, kspace: Varchar<40>) -> EResult;
    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>>;
    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>>;

    async fn chnot_overwrite_blocks(&self, blocks: Vec<ChnotBlock>) -> EResult;

    async fn ensure_table_chnot(&self) -> EResult;
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite_record(
        &self,
        req: KReq<ChnotOverwriteRecordReq>,
    ) -> AResult<ChnotOverwriteRecordRsp> {
        expand_mt_branch!(self.chnot_overwrite_record(req))
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Chnot>> {
        expand_mt_branch!(self.chnot_query(req))
    }

    async fn chnot_overwrite_meta(
        &self,
        req: KReq<ChnotOverwriteMetaReq>,
    ) -> AResult<ChnotOverwriteMetaRsp> {
        expand_mt_branch!(self.chnot_overwrite_meta(req))
    }

    async fn ensure_table_chnot(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot())
    }

    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>> {
        expand_mt_branch!(self.chnot_tag_query(req))
    }

    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>> {
        expand_mt_branch!(self.chnot_tag_names(req))
    }

    async fn chnot_tag_update_all(&self, kspace: Varchar<40>) -> EResult {
        expand_mt_branch!(self.chnot_tag_update_all(kspace))
    }

    async fn chnot_query_kind_rel(
        &self,
        chnot_meta_otid: KReq<ChnotKindRelQueryReq>,
    ) -> AResult<ChnotKindRelQueryRsp> {
        expand_mt_branch!(self.chnot_query_kind_rel(chnot_meta_otid))
    }

    async fn chnot_overwrite_blocks(&self, blocks: Vec<ChnotBlock>) -> EResult {
        expand_mt_branch!(self.chnot_overwrite_blocks(blocks))
    }
}
