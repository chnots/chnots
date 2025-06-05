use chin_tools::{AResult, EResult};

use crate::{expand_mt_branch, model::dto::KReq, MapperType};

use super::*;

pub(crate) trait KTabMapper {
    async fn ktab_overwrite_meta(
        &self,
        req: KReq<KTabMetaOverwriteReq>,
    ) -> AResult<KTabMetaOverwriteRsp>;

    async fn ktab_overwrite_cells(
        &self,
        req: KReq<KTabCellsOverwriteReq>,
    ) -> AResult<KTabCellsOverwriteRsp>;

    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabMetaQueryReq>,
    ) -> AResult<KTabMetaQueryRsp>;
    
    async fn ktab_query_table_data(
        &self,
        req: KReq<KTabRowsQueryReq>,
    ) -> AResult<KTabRowsQueryRsp>;

    async fn ensure_ktab_tables(&self) -> EResult;
}

impl KTabMapper for MapperType {
    async fn ktab_overwrite_meta(
        &self,
        req: KReq<KTabMetaOverwriteReq>,
    ) -> AResult<KTabMetaOverwriteRsp> {
        expand_mt_branch!(self.ktab_overwrite_meta(req))
    }

    async fn ktab_overwrite_cells(
        &self,
        req: KReq<KTabCellsOverwriteReq>,
    ) -> AResult<KTabCellsOverwriteRsp> {
        expand_mt_branch!(self.ktab_overwrite_cells(req))
    }

    async fn ktab_query_table_meta(
        &self,
        req: KReq<KTabMetaQueryReq>,
    ) -> AResult<KTabMetaQueryRsp> {
        expand_mt_branch!(self.ktab_query_table_meta(req))
    }

    async fn ktab_query_table_data(
        &self,
        req: KReq<KTabRowsQueryReq>,
    ) -> AResult<KTabRowsQueryRsp> {
        expand_mt_branch!(self.ktab_query_table_data(req))
    }

    async fn ensure_ktab_tables(&self) -> EResult {
        expand_mt_branch!(self.ensure_ktab_tables())
    }
}
