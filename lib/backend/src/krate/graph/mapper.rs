use super::*;
use crate::{MapperType, expand_mt_branch, model::dto::KReq};
use chin_tools::{AResult, EResult};

pub trait GraphMapper {
    async fn excalidraw_fetch(&self, req: KReq<ExcalidrawFetchReq>) -> AResult<ExcalidrawFetchRsp>;

    async fn excalidraw_commit(
        &self,
        req: KReq<ExcalidrawCommitReq>,
    ) -> AResult<ExcalidrawCommitRsp>;

    async fn excalidraw_library_fetch(
        &self,
        req: KReq<ExcalidrawLibraryFetchReq>,
    ) -> AResult<ExcalidrawLibraryFetchRsp>;

    async fn excalidraw_library_commit(
        &self,
        req: KReq<ExcalidrawLibraryCommitReq>,
    ) -> AResult<ExcalidrawLibraryCommitRsp>;

    async fn mind_elixir_fetch(&self, req: KReq<MindElixirLoadReq>) -> AResult<MindElixirLoadRsp>;

    async fn mind_elixir_commit(
        &self,
        req: KReq<MindElixirCommitReq>,
    ) -> AResult<MindElixirCommitRsp>;

    async fn excalidraw_history_list(
        &self,
        req: KReq<GraphHistoryListReq>,
    ) -> AResult<GraphHistoryListRsp>;

    async fn excalidraw_history_fetch(
        &self,
        req: KReq<ExcalidrawHistoryFetchReq>,
    ) -> AResult<ExcalidrawHistoryFetchRsp>;

    async fn excalidraw_history_apply(
        &self,
        req: KReq<ExcalidrawHistoryApplyReq>,
    ) -> AResult<ExcalidrawHistoryApplyRsp>;

    async fn mind_elixir_history_list(
        &self,
        req: KReq<GraphHistoryListReq>,
    ) -> AResult<GraphHistoryListRsp>;

    async fn mind_elixir_history_fetch(
        &self,
        req: KReq<MindElixirHistoryFetchReq>,
    ) -> AResult<MindElixirHistoryFetchRsp>;

    async fn mind_elixir_history_apply(
        &self,
        req: KReq<MindElixirHistoryApplyReq>,
    ) -> AResult<MindElixirHistoryApplyRsp>;

    async fn ensure_table_graph(&self) -> EResult;
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

    async fn excalidraw_library_fetch(
        &self,
        req: KReq<ExcalidrawLibraryFetchReq>,
    ) -> AResult<ExcalidrawLibraryFetchRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_library_fetch(req).await,
        }
    }

    async fn excalidraw_library_commit(
        &self,
        req: KReq<ExcalidrawLibraryCommitReq>,
    ) -> AResult<ExcalidrawLibraryCommitRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_library_commit(req).await,
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

    async fn excalidraw_history_list(
        &self,
        req: KReq<GraphHistoryListReq>,
    ) -> AResult<GraphHistoryListRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_history_list(req).await,
        }
    }

    async fn excalidraw_history_fetch(
        &self,
        req: KReq<ExcalidrawHistoryFetchReq>,
    ) -> AResult<ExcalidrawHistoryFetchRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_history_fetch(req).await,
        }
    }

    async fn excalidraw_history_apply(
        &self,
        req: KReq<ExcalidrawHistoryApplyReq>,
    ) -> AResult<ExcalidrawHistoryApplyRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.excalidraw_history_apply(req).await,
        }
    }

    async fn mind_elixir_history_list(
        &self,
        req: KReq<GraphHistoryListReq>,
    ) -> AResult<GraphHistoryListRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.mind_elixir_history_list(req).await,
        }
    }

    async fn mind_elixir_history_fetch(
        &self,
        req: KReq<MindElixirHistoryFetchReq>,
    ) -> AResult<MindElixirHistoryFetchRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.mind_elixir_history_fetch(req).await,
        }
    }

    async fn mind_elixir_history_apply(
        &self,
        req: KReq<MindElixirHistoryApplyReq>,
    ) -> AResult<MindElixirHistoryApplyRsp> {
        match self {
            MapperType::KDb(kdb) => kdb.mind_elixir_history_apply(req).await,
        }
    }

    async fn ensure_table_graph(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_graph())
    }
}
