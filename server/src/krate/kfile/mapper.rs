use super::*;
use crate::{expand_mt_branch, model::dto::KReq, MapperType};
use chin_tools::{AResult, EResult};

pub(crate) trait KFileDeserializeMapper {
    fn to_inline_kfile(self) -> AResult<InlineKFile>;
    fn to_kfile_meta(self) -> AResult<KFileMeta>;
}

pub(crate) trait KFileDumpMapper {
    async fn dump_kfile() -> EResult;
    async fn dump_inline_kfile() -> EResult;
}

pub(crate) trait KFileMapper {
    async fn insert_kfile(&self, kfile: KFileMeta) -> anyhow::Result<()>;
    async fn query_kfile_meta(&self, req: QueryKFileReq) -> anyhow::Result<QueryKFileMetaRsp>;
    async fn query_kfile_meta_by_sid(&self, sid: &str) -> anyhow::Result<QueryKFileMetaRsp>;

    ///
    /// Try to insert inline kfile.
    ///
    /// Inline kfile could keep history if record with archor flag.
    /// If the new version record time is long enough from the old archor,
    /// insert it with archor flag.
    ///
    async fn insert_inline_kfile(
        &self,
        req: KReq<InsertInlineKFileReq>,
    ) -> anyhow::Result<InsertInlineKFileRsp>;

    async fn query_inline_kfile(
        &self,
        req: KReq<QueryInlineKFileReq>,
    ) -> anyhow::Result<QueryInlineKFileRsp>;

    async fn ensure_table_kfile(&self) -> EResult;
    async fn ensure_table_inline_kfile(&self) -> EResult;
}

impl KFileMapper for MapperType {
    async fn insert_kfile(&self, kfile: KFileMeta) -> anyhow::Result<()> {
        expand_mt_branch!(self.insert_kfile(kfile))
    }

    async fn ensure_table_kfile(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kfile())
    }

    async fn insert_inline_kfile(
        &self,
        req: KReq<InsertInlineKFileReq>,
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

    async fn query_kfile_meta(&self, req: QueryKFileReq) -> anyhow::Result<QueryKFileMetaRsp> {
        expand_mt_branch!(self.query_kfile_meta(req))
    }
    
    async fn query_kfile_meta_by_sid(&self, sid: &str) -> anyhow::Result<QueryKFileMetaRsp> {
        expand_mt_branch!(self.query_kfile_meta_by_sid(sid))
    }
}
