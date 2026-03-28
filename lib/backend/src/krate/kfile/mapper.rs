use super::*;
use crate::{MapperType, expand_mt_branch, model::dto::KReq};
use chin_sql::str_type::Varchar;
use chin_tools::{AResult, EResult};

pub trait KFileMapper {
    async fn po_insert_kfile_meta(&self, kfile: KFileMeta) -> anyhow::Result<()>;
    async fn po_inline_kfile_commit(&self, pos: Vec<InlineKFile>) -> AResult<usize>;
    async fn po_inline_kfile_list(
        &self,
        sids: Vec<Varchar<100>>,
    ) -> anyhow::Result<Vec<InlineKFile>>;

    async fn query_kfile_meta(&self, req: KfileMetaFetchReq) -> anyhow::Result<KfileMetaFetchRsp>;
    #[allow(dead_code)]
    async fn query_kfile_meta_by_sid(&self, sid: Varchar<100>)
    -> anyhow::Result<KfileMetaFetchRsp>;

    ///
    /// Try to insert inline kfile.
    ///
    /// Inline kfile could keep history if record with archor flag.
    /// If the new version record time is long enough from the old archor,
    /// insert it with archor flag.
    ///
    async fn insert_inline_kfile(
        &self,
        req: KReq<InlineKFileUploadReq>,
    ) -> anyhow::Result<InlineKFileUploadRsp>;

    async fn query_inline_kfile(
        &self,
        req: KReq<InlineKFileDownloadReq>,
    ) -> anyhow::Result<InlineKFileDownloadRsp>;

    async fn kfile_history_list(
        &self,
        req: KReq<KfileHistoryListReq>,
    ) -> AResult<KfileHistoryListRsp>;

    async fn kfile_history_fetch(
        &self,
        req: KReq<KfileHistoryFetchReq>,
    ) -> AResult<KfileHistoryFetchRsp>;

    async fn kfile_history_apply(
        &self,
        req: KReq<KfileHistoryApplyReq>,
    ) -> AResult<KfileHistoryApplyRsp>;

    async fn ensure_table_kfile(&self) -> EResult;
}

impl KFileMapper for MapperType {
    async fn po_insert_kfile_meta(&self, kfile: KFileMeta) -> anyhow::Result<()> {
        expand_mt_branch!(self.po_insert_kfile_meta(kfile))
    }

    async fn po_inline_kfile_commit(&self, req: Vec<InlineKFile>) -> AResult<usize> {
        expand_mt_branch!(self.po_inline_kfile_commit(req))
    }

    async fn po_inline_kfile_list(
        &self,
        sids: Vec<Varchar<100>>,
    ) -> anyhow::Result<Vec<InlineKFile>> {
        expand_mt_branch!(self.po_inline_kfile_list(sids))
    }

    async fn ensure_table_kfile(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kfile())
    }

    async fn insert_inline_kfile(
        &self,
        req: KReq<InlineKFileUploadReq>,
    ) -> anyhow::Result<InlineKFileUploadRsp> {
        expand_mt_branch!(self.insert_inline_kfile(req))
    }

    async fn query_inline_kfile(
        &self,
        req: KReq<InlineKFileDownloadReq>,
    ) -> anyhow::Result<InlineKFileDownloadRsp> {
        expand_mt_branch!(self.query_inline_kfile(req))
    }

    async fn query_kfile_meta(&self, req: KfileMetaFetchReq) -> anyhow::Result<KfileMetaFetchRsp> {
        expand_mt_branch!(self.query_kfile_meta(req))
    }

    async fn query_kfile_meta_by_sid(
        &self,
        sid: Varchar<100>,
    ) -> anyhow::Result<KfileMetaFetchRsp> {
        expand_mt_branch!(self.query_kfile_meta_by_sid(sid))
    }

    async fn kfile_history_list(
        &self,
        req: KReq<KfileHistoryListReq>,
    ) -> AResult<KfileHistoryListRsp> {
        expand_mt_branch!(self.kfile_history_list(req))
    }

    async fn kfile_history_fetch(
        &self,
        req: KReq<KfileHistoryFetchReq>,
    ) -> AResult<KfileHistoryFetchRsp> {
        expand_mt_branch!(self.kfile_history_fetch(req))
    }

    async fn kfile_history_apply(
        &self,
        req: KReq<KfileHistoryApplyReq>,
    ) -> AResult<KfileHistoryApplyRsp> {
        expand_mt_branch!(self.kfile_history_apply(req))
    }
}
