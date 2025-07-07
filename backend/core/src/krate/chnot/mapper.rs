use chin_sql::str_type::Varchar;
use chin_tools::{AResult, EResult};

use crate::{expand_mt_branch, model::dto::KReq, MapperType, RecordCallbackType};

use super::*;

pub(crate) trait ChnotDeserializeMapper {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata>;
    fn to_chnot_record(self) -> AResult<ChnotRecord>;
    fn to_chnot_tag(self) -> AResult<ChnotTag>;
    fn to_chnot_kind_rel(self) -> AResult<ChnotKindRel>;
}

pub(crate) trait ChnotDumpMapper {
    async fn dump_chnot_meta(&self, callback: &RecordCallbackType) -> EResult;
    async fn dump_chnot_record(&self, callback: &RecordCallbackType) -> EResult;
    async fn dump_chnot_tag(&self, callback: &RecordCallbackType) -> EResult;

    async fn dump_chnot(&self, callback: &RecordCallbackType) -> EResult {
        self.dump_chnot_meta(callback).await?;
        self.dump_chnot_record(callback).await?;
        self.dump_chnot_tag(callback).await?;

        Ok(())
    }
}

pub trait ChnotMapper {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp>;
    async fn chnot_archive(&self, req: KReq<ChnotArchiveReq>) -> AResult<ChnotArchiveRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Chnot>>;
    async fn chnot_query_kind_rel(
        &self,
        req: KReq<ChnotKindRelQueryReq>,
    ) -> AResult<ChnotKindRelQueryRsp>;
    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn chnot_tag_update_all(&self, kspace: Varchar<40>) -> EResult;
    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>>;
    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>>;

    async fn ensure_table_chnot(&self) -> EResult;
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        expand_mt_branch!(self.chnot_overwrite(req))
    }

    async fn chnot_archive(&self, req: KReq<ChnotArchiveReq>) -> AResult<ChnotArchiveRsp> {
        expand_mt_branch!(self.chnot_archive(req))
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Chnot>> {
        expand_mt_branch!(self.chnot_query(req))
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        expand_mt_branch!(self.chnot_update(req))
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
}
