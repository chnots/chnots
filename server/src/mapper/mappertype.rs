use chin_tools::{utils::sort_util, AResult, EResult};

use crate::{llmchat::mapper::LLMChatMapper, model::{
    db::{chnot::ChnotTag, kfile::KFile, workspace::WorkspaceRelation},
    dto::{
        ctable::*,
        kfile::{
            InsertInlineKFileReq, InsertInlineKFileRsp, KVOverwriteReq, KVOverwriteRsp,
            KVQueryReq, KVQueryRsp, QueryInlineKFileReq, QueryInlineKFileRsp,
        },
    },
}};

use super::{
    db::{postgres::Postgres, sqlite::Sqlite},
    dump::RecordCallbackType,
    ChinTableMapper, ChnotDeletionRsp, ChnotMapper, ChnotOverwriteReq, ChnotOverwriteRsp,
    DumpMapper, KVMapper, MapperConfig, MapperType, WorkspaceMapper, KFileMapper,
};

use crate::model::{
    db::workspace::WorkspaceRecord,
    dto::{chnot::*, KReq},
};

impl Into<AResult<MapperType>> for MapperConfig {
    fn into(self) -> AResult<MapperType> {
        match self {
            MapperConfig::Postgres(config) => {
                let pg = Postgres::new(config)?;
                Ok(MapperType::KDb(super::db::KDb::Postgres(pg)))
            }
            MapperConfig::Sqlite(config) => {
                let sqlite = Sqlite::new(config)?;
                Ok(MapperType::KDb(super::db::KDb::Sqlite(sqlite)))
            }
        }
    }
}

impl MapperType {
    pub(crate) async fn ensure_tables(&self) -> EResult {
        self.ensure_table_chnot_record().await?;
        self.ensure_table_workspace_record().await?;
        self.ensure_table_workspace_relation().await?;
        self.ensure_table_chnot_metadata().await?;
        self.ensure_table_chnot_tag().await?;
        self.ensure_table_kfile().await?;
        self.ensure_table_inline_kfile().await?;
        self.ensure_table_kv().await?;

        self.ensure_table_llm_chat().await?;

        self.ensure_ctable_tables().await?;

        Ok(())
    }
}

#[macro_export]
macro_rules! expand_mt_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            MapperType::KDb(db) => db.$method($($arg),*).await,
        }
    };
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

    async fn chnot_tag_insert(&self, req: crate::model::db::chnot::ChnotTag) -> EResult {
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

impl KFileMapper for MapperType {
    async fn insert_kfile(&self, kfile: &KFile) -> anyhow::Result<KFile> {
        expand_mt_branch!(self.insert_kfile(kfile))
    }

    async fn query_kfile_by_id(&self, id: &str) -> anyhow::Result<KFile> {
        expand_mt_branch!(self.query_kfile_by_id(id))
    }

    async fn ensure_table_kfile(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kfile())
    }

    async fn insert_inline_kfile(
        &self,
        req: &KReq<InsertInlineKFileReq>,
    ) -> anyhow::Result<InsertInlineKFileRsp> {
        expand_mt_branch!(self.insert_inline_kfile(req))
    }

    async fn query_inline_kfile(
        &self,
        req: KReq<QueryInlineKFileReq>,
    ) -> anyhow::Result<QueryInlineKFileRsp> {
        expand_mt_branch!(self.query_inline_kfile(req))
    }

    async fn ensure_table_inline_kfile(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_inline_kfile())
    }
    
}

impl WorkspaceMapper for MapperType {
    async fn read_all_workspaces(&self) -> AResult<Vec<WorkspaceRecord>> {
        expand_mt_branch!(self.read_all_workspaces())
    }

    async fn read_all_workspace_relations(&self) -> AResult<Vec<WorkspaceRelation>> {
        expand_mt_branch!(self.read_all_workspace_relations())
    }

    async fn ensure_table_workspace_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_workspace_record())
    }

    async fn ensure_table_workspace_relation(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_workspace_relation())
    }
}

impl MapperType {
    pub(crate) async fn dump_and_callback(&self, writer: &RecordCallbackType) -> EResult {
        expand_mt_branch!(self.dump_and_callback(writer))
    }
}

impl KVMapper for MapperType {
    async fn kv_overwrite(&self, req: KReq<KVOverwriteReq>) -> AResult<KVOverwriteRsp> {
        expand_mt_branch!(self.kv_overwrite(req))
    }

    async fn kv_query(&self, req: KReq<KVQueryReq>) -> AResult<KVQueryRsp> {
        expand_mt_branch!(self.kv_query(req))
    }

    async fn ensure_table_kv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kv())
    }

    async fn kv_delete(&self, req: KReq<super::KVDeleteReq>) -> AResult<super::KVDeleteRsp> {
        expand_mt_branch!(self.kv_delete(req))
    }
}

impl ChinTableMapper for MapperType {
    async fn ctable_overwrite_meta(
        &self,
        req: KReq<CTableOverwriteMetaReq>,
    ) -> AResult<CTableOverwriteMetaRsp> {
        expand_mt_branch!(self.ctable_overwrite_meta(req))
    }

    async fn ctable_overwrite_row(
        &self,
        req: KReq<CTableOverwriteRowReq>,
    ) -> AResult<CTableOverwriteRowRsp> {
        expand_mt_branch!(self.ctable_overwrite_row(req))
    }

    async fn ctable_overwrite_cell(
        &self,
        req: KReq<CTableOverwriteCellReq>,
    ) -> AResult<CTableOverwriteCellRsp> {
        expand_mt_branch!(self.ctable_overwrite_cell(req))
    }

    async fn ctable_query_row(&self, req: KReq<CTableQueryRowReq>) -> AResult<CTableQueryRowRsp> {
        expand_mt_branch!(self.ctable_query_row(req))
    }

    async fn ctable_query_table_meta(
        &self,
        req: KReq<CTableQueryTableMetaReq>,
    ) -> AResult<CTableQueryTableMetaRsp> {
        expand_mt_branch!(self.ctable_query_table_meta(req))
    }

    async fn ctable_query_table_data(
        &self,
        req: KReq<CTableQueryTableDataReq>,
    ) -> AResult<CTableQueryTableDataRsp> {
        expand_mt_branch!(self.ctable_query_table_data(req))
    }

    async fn ensure_ctable_tables(&self) -> EResult {
        expand_mt_branch!(self.ensure_ctable_tables())
    }
}
