use std::{collections::HashSet, path::PathBuf};

use crate::{
    app::ShareAppState,
    krate::{
        graph::{GetKeys, GraphData, GraphMeta, GraphMetaEnum},
        sync::{
            dto::{SyncDataDto, SyncSidPoGenericDto},
            filedumper::StartType,
            mapper::SyncMapper,
            networksync::OtidRelatedWorker,
            po::SyncEndpoint,
        },
    },
};
use chin_sql::str_type::Varchar;
use chin_tools::EResult;

struct GraphDataSyncWorker {
    app: ShareAppState,
}
impl OtidRelatedWorker<GraphMeta> for GraphDataSyncWorker {
    async fn before_send(&self, endpoint: &SyncEndpoint, arg: &SyncDataDto<GraphMeta>) -> EResult {
        let mut metas: Vec<Varchar<100>> = vec![];
        for ele in &arg.cmds {
            if let crate::krate::sync::dto::SyncDataOperation::Push { data, hist: _ } = ele {
                let keys = GraphMetaEnum::try_from(data)?.get_keys();
                let keys: Result<Vec<Varchar<100>>, chin_sql::ChinSqlError> = keys
                    .iter()
                    .map(|e: &String| Varchar::<100>::try_from(e.clone()))
                    .collect();
                metas.extend(keys?);
            }
        }
        let data: Vec<GraphData> = self.app.po_sid_sync_list(metas).await?;

        self.app
            .sync_sid_po_commit_tx(endpoint, SyncSidPoGenericDto { pos: data })
            .await?;

        Ok(())
    }

    async fn before_merge(&self, endpoint: &SyncEndpoint, arg: &SyncDataDto<GraphMeta>) -> EResult {
        let mut metas: Vec<Varchar<100>> = vec![];
        for ele in &arg.cmds {
            if let crate::krate::sync::dto::SyncDataOperation::Push { data, hist: _ } = ele {
                let keys = GraphMetaEnum::try_from(data)?.get_keys();
                let keys: Result<Vec<Varchar<100>>, chin_sql::ChinSqlError> = keys
                    .iter()
                    .map(|e: &String| Varchar::<100>::try_from(e.clone()))
                    .collect();
                metas.extend(keys?);
            }
        }

        let data = self
            .app
            .sync_sid_po_list_tx::<100, GraphData>(endpoint, metas)
            .await?;
        self.app.po_sid_sync_commit(data).await?;

        Ok(())
    }
}

impl ShareAppState {
    pub async fn dump_graph_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        let mapper = &self.mapper;

        mapper
            .dump_to_file::<&PathBuf, GraphMeta>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_graph(&self, endpoint: &crate::krate::sync::po::SyncEndpoint) -> EResult {
        self.sync_one_otid_table_with_worker(endpoint, &GraphDataSyncWorker { app: self.clone() })
            .await?;

        Ok(())
    }
}
