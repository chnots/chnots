use chin_tools::{AResult, EResult};

use crate::{expand_mt_branch, model::dto::KReq, MapperType};

use super::*;

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
