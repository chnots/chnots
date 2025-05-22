use chin_tools::{AResult, EResult};

use crate::{expand_mt_branch, model::dto::KReq, MapperType};

use super::*;

pub(crate) trait KTabMapper {
    async fn ktab_overwrite_meta(
        &self,
        req: KReq<KTabOverwriteMetaReq>,
    ) -> AResult<KTabOverwriteMetaRsp>;
    async fn ktab_overwrite_row(
        &self,
        req: KReq<KTabOverwriteRowReq>,
    ) -> AResult<KTabOverwriteRowRsp>;
    async fn ktab_overwrite_cell(
        &self,
        req: KReq<KTabOverwriteCellReq>,
    ) -> AResult<KTabOverwriteCellRsp>;
    async fn ktab_query_row(&self, req: KReq<KTabQueryRowReq>) -> AResult<KTabQueryRowRsp>;
    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabQueryTableMetaReq>,
    ) -> AResult<KTabQueryTableMetaRsp>;
    async fn ktab_query_table_data(
        &self,
        req: KReq<KTabQueryTableDataReq>,
    ) -> AResult<KTabQueryTableDataRsp>;

    async fn ensure_ktab_tables(&self) -> EResult;
}

impl KTabMapper for MapperType {
    async fn ktab_overwrite_meta(
        &self,
        req: KReq<KTabOverwriteMetaReq>,
    ) -> AResult<KTabOverwriteMetaRsp> {
        expand_mt_branch!(self.ktab_overwrite_meta(req))
    }

    async fn ktab_overwrite_row(
        &self,
        req: KReq<KTabOverwriteRowReq>,
    ) -> AResult<KTabOverwriteRowRsp> {
        expand_mt_branch!(self.ktab_overwrite_row(req))
    }

    async fn ktab_overwrite_cell(
        &self,
        req: KReq<KTabOverwriteCellReq>,
    ) -> AResult<KTabOverwriteCellRsp> {
        expand_mt_branch!(self.ktab_overwrite_cell(req))
    }

    async fn ktab_query_row(&self, req: KReq<KTabQueryRowReq>) -> AResult<KTabQueryRowRsp> {
        expand_mt_branch!(self.ktab_query_row(req))
    }

    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabQueryTableMetaReq>,
    ) -> AResult<KTabQueryTableMetaRsp> {
        expand_mt_branch!(self.ktab_query_table_meta(req))
    }

    async fn ktab_query_table_data(
        &self,
        req: KReq<KTabQueryTableDataReq>,
    ) -> AResult<KTabQueryTableDataRsp> {
        expand_mt_branch!(self.ktab_query_table_data(req))
    }

    async fn ensure_ktab_tables(&self) -> EResult {
        expand_mt_branch!(self.ensure_ktab_tables())
    }
}
