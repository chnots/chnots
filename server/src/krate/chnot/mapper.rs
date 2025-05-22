use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    mapper::db::{KDbRow, KDbRowBehavier},
    model::dto::KReq,
    MapperType, RecordCallbackType,
};

use super::*;

pub(crate) trait ChnotDeserializeMapper {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata>;
    fn to_chnot_record(self) -> AResult<ChnotRecord>;
    fn to_chnot_tag(self) -> AResult<ChnotTag>;
}

pub(crate) trait LLMChatDumpMapper {
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

impl ChnotDeserializeMapper for KDbRow<'_> {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: self.try_get(ChnotMetadata::ID)?,
            workspace: self.try_get(ChnotMetadata::WORKSPACE)?,
            kind: self.try_get(ChnotMetadata::KIND)?,
            pin_time: self.try_get(ChnotMetadata::PIN_TIME)?,
            delete_time: self.try_get(ChnotMetadata::DELETE_TIME)?,
            update_time: self.try_get(ChnotMetadata::UPDATE_TIME)?,
            insert_time: self.try_get(ChnotMetadata::INSERT_TIME)?,
            archive_time: self.try_get(ChnotMetadata::ARCHIVE_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: self.try_get(ChnotRecord::ID)?,
            meta_id: self.try_get(ChnotRecord::META_ID)?,
            content: self.try_get(ChnotRecord::CONTENT)?,
            omit_time: self.try_get(ChnotRecord::OMIT_TIME)?,
            insert_time: self.try_get(ChnotRecord::INSERT_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            id: self.try_get(ChnotTag::ID)?,
            workspace: self.try_get(ChnotTag::WORKSPACE)?,
            tag: self.try_get(ChnotTag::TAG)?,
            chnot_meta_id: self.try_get(ChnotTag::CHNOT_META_ID)?,
            insert_time: self.try_get(ChnotTag::INSERT_TIME)?,
            category: ChnotTagType::Common,
        };
        Ok(obj)
    }
}

pub(crate) trait ChnotMapper {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp>;
    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>>;
    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn chnot_tag_update_single_chnot(
        &self,
        content: &str,
        meta_id: &str,
        workspace: &str,
    ) -> EResult;
    async fn chnot_tag_update_all(&self, workspace: &str) -> EResult;
    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>>;
    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>>;
    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult;
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

    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult {
        expand_mt_branch!(self.chnot_tag_insert(req))
    }

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        expand_mt_branch!(self.chnot_tag_delete(chnot_meta_ids))
    }

    async fn chnot_tag_update_single_chnot(
        &self,
        content: &str,
        meta_id: &str,
        workspace: &str,
    ) -> EResult {
        expand_mt_branch!(self.chnot_tag_update_single_chnot(content, meta_id, workspace))
    }

    async fn chnot_tag_update_all(&self, workspace: &str) -> EResult {
        expand_mt_branch!(self.chnot_tag_update_all(workspace))
    }
}
