use chin_tools::{AResult, EResult};

use crate::{expand_mt_branch, model::dto::KReq, MapperType, RecordCallbackType};

use super::*;

pub(crate) trait ChnotDeserializeMapper {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata>;
    fn to_chnot_record(self) -> AResult<ChnotRecord>;
    fn to_chnot_tag(self) -> AResult<ChnotTag>;
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

pub(crate) trait ChnotMapper {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp>;
    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>>;
    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn chnot_tag_update_all(&self, kspace: &str) -> EResult;
    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>>;
    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>>;
    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult;

    async fn ensure_table_chnot_record(&self) -> EResult;
    async fn ensure_table_chnot_metadata(&self) -> EResult;
    async fn ensure_table_chnot_tag(&self) -> EResult;

    async fn ensure_table_chnot(&self) -> EResult {
        self.ensure_table_chnot_record().await?;
        self.ensure_table_chnot_metadata().await?;
        self.ensure_table_chnot_tag().await?;

        Ok(())
    }
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        expand_mt_branch!(self.chnot_overwrite(req))
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        expand_mt_branch!(self.chnot_delete(req))
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        expand_mt_branch!(self.chnot_query(req))
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        expand_mt_branch!(self.chnot_update(req))
    }

    async fn ensure_table_chnot_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot_record())
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot_metadata())
    }

    async fn ensure_table_chnot_tag(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_chnot_tag())
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

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        expand_mt_branch!(self.chnot_tag_delete(chnot_meta_ids))
    }

    async fn chnot_tag_update_all(&self, kspace: &str) -> EResult {
        expand_mt_branch!(self.chnot_tag_update_all(kspace))
    }
}
