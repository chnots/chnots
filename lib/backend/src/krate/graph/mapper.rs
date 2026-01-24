use super::*;
use crate::{MapperType, expand_mt_branch, model::dto::KReq};
use chin_sql::{str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};

pub trait GraphMapper {
    async fn excalidraw_fetch(&self, req: KReq<ExcalidrawFetchReq>) -> AResult<ExcalidrawFetchRsp>;

    async fn excalidraw_commit(
        &self,
        req: KReq<ExcalidrawCommitReq>,
    ) -> AResult<ExcalidrawCommitRsp>;

    async fn mind_elixir_fetch(&self, req: KReq<MindElixirLoadReq>) -> AResult<MindElixirLoadRsp>;

    async fn mind_elixir_commit(
        &self,
        req: KReq<MindElixirCommitReq>,
    ) -> AResult<MindElixirCommitRsp>;
}

impl GraphMapper for MapperType {
    async fn excalidraw_fetch(&self, req: KReq<ExcalidrawFetchReq>) -> AResult<ExcalidrawFetchRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_fetch(req).await,
        }
    }

    async fn excalidraw_commit(
        &self,
        req: KReq<ExcalidrawCommitReq>,
    ) -> AResult<ExcalidrawCommitRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_commit(req).await,
        }
    }

    async fn mind_elixir_fetch(&self, req: KReq<MindElixirLoadReq>) -> AResult<MindElixirLoadRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.mind_elixir_fetch(req).await,
        }
    }

    async fn mind_elixir_commit(
        &self,
        req: KReq<MindElixirCommitReq>,
    ) -> AResult<MindElixirCommitRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.mind_elixir_commit(req).await,
        }
    }
}
