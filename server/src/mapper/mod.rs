pub(crate) mod db;
pub(crate) mod dump;
pub(crate) mod mappertype;

use chin_tools::{AResult, EResult};
use db::{postgres::PostgresConfig, sqlite::SqliteConfig, KDb};
use dump::RecordCallbackType;
use serde::Deserialize;

use crate::model::{
    db::{
        chnot::{ChnotMetadata, ChnotRecord, ChnotTag},
        workspace::{WorkspaceRecord, WorkspaceRelation},
        kfile::{InlineKFile, KFile, KTV},
    },
    dto::{chnot::*, ctable::*,  kfile::*, KReq},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub(crate) enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
    #[serde(rename = "sqlite")]
    Sqlite(SqliteConfig),
}

pub(crate) enum MapperType {
    KDb(KDb),
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
}

pub(crate) trait KFileMapper {
    async fn insert_kfile(&self, kfile: &KFile) -> anyhow::Result<KFile>;
    async fn query_kfile_by_id(&self, id: &str) -> anyhow::Result<KFile>;
    ///
    /// Try to insert inline kfile.
    ///
    /// Inline kfile could keep history if record with archor flag.
    /// If the new version record time is long enough from the old archor,
    /// insert it with archor flag.
    ///
    async fn insert_inline_kfile(
        &self,
        req: &KReq<InsertInlineKFileReq>,
    ) -> anyhow::Result<InsertInlineKFileRsp>;
    async fn query_inline_kfile(
        &self,
        req: KReq<QueryInlineKFileReq>,
    ) -> anyhow::Result<QueryInlineKFileRsp>;

    async fn ensure_table_kfile(&self) -> EResult;
    async fn ensure_table_inline_kfile(&self) -> EResult;
}

pub(crate) trait WorkspaceMapper {
    async fn read_all_workspaces(&self) -> AResult<Vec<WorkspaceRecord>>;
    async fn read_all_workspace_relations(&self) -> AResult<Vec<WorkspaceRelation>>;

    async fn ensure_table_workspace_record(&self) -> EResult;
    async fn ensure_table_workspace_relation(&self) -> EResult;
}



pub(crate) trait KVMapper {
    async fn kv_overwrite(&self, req: KReq<KVOverwriteReq>) -> AResult<KVOverwriteRsp>;
    async fn kv_query(&self, req: KReq<KVQueryReq>) -> AResult<KVQueryRsp>;
    async fn kv_delete(&self, req: KReq<KVDeleteReq>) -> AResult<KVDeleteRsp>;
    async fn ensure_table_kv(&self) -> EResult;
}

pub(crate) trait DumpMapper {
    type RowType<'a>;

    async fn dump_and_callback(&self, callback: &RecordCallbackType) -> EResult;
}

pub(crate) trait ChinTableMapper {
    async fn ctable_overwrite_meta(
        &self,
        req: KReq<CTableOverwriteMetaReq>,
    ) -> AResult<CTableOverwriteMetaRsp>;
    async fn ctable_overwrite_row(
        &self,
        req: KReq<CTableOverwriteRowReq>,
    ) -> AResult<CTableOverwriteRowRsp>;
    async fn ctable_overwrite_cell(
        &self,
        req: KReq<CTableOverwriteCellReq>,
    ) -> AResult<CTableOverwriteCellRsp>;
    async fn ctable_query_row(&self, req: KReq<CTableQueryRowReq>) -> AResult<CTableQueryRowRsp>;
    async fn ctable_query_table_meta(
        &self,
        req: KReq<CTableQueryTableMetaReq>,
    ) -> AResult<CTableQueryTableMetaRsp>;
    async fn ctable_query_table_data(
        &self,
        req: KReq<CTableQueryTableDataReq>,
    ) -> AResult<CTableQueryTableDataRsp>;

    async fn ensure_ctable_tables(&self) -> EResult;
}

pub(crate) trait DeserializeMapper {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata>;
    fn to_chnot_record(self) -> AResult<ChnotRecord>;
    fn to_chnot_tag(self) -> AResult<ChnotTag>;

    fn to_workspace_record(self) -> AResult<WorkspaceRecord>;
    fn to_workspace_relation(self) -> AResult<WorkspaceRelation>;

    fn to_kfile(self) -> AResult<KFile>;
    fn to_inline_kfile(self) -> AResult<InlineKFile>;

    fn to_kv(self) -> AResult<KTV>;
}
