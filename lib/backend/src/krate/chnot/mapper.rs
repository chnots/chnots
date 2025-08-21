use chin_sql::str_type::Varchar;
use chin_tools::{AResult, EResult};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait ChnotMapper {
    async fn chnot_overwrite_records(
        &self,
        req: KReq<ChnotOverwriteRecordReq>,
    ) -> AResult<ChnotOverwriteRecordRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Chnot>>;
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

    async fn chnot_detail(&self, req: KReq<ChnotDetailReq>) -> AResult<ChnotDetailRsp>;

    async fn ensure_table_chnot(&self) -> EResult;
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite_records(
        &self,
        req: KReq<ChnotOverwriteRecordReq>,
    ) -> AResult<ChnotOverwriteRecordRsp> {
        expand_mt_branch!(self.chnot_overwrite_records(req))
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

    async fn chnot_detail(&self, req: KReq<ChnotDetailReq>) -> AResult<ChnotDetailRsp> {
        expand_mt_branch!(self.chnot_detail(req))
    }
}
