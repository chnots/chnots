use super::*;
use crate::{expand_mt_branch, model::dto::KReq, MapperType};
use chin_tools::{AResult, EResult};

pub(crate) trait KFileDeserializeMapper {
    fn to_kfile(self) -> AResult<KFile>;
    fn to_inline_kfile(self) -> AResult<InlineKFile>;
}

pub(crate) trait KFileDumpMapper {
    async fn dump_kfile() -> EResult;
    async fn dump_inline_kfile() -> EResult;
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
