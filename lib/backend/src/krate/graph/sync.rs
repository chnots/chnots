use std::path::PathBuf;

use crate::{
    app::ShareAppState,
    krate::{graph::GraphMeta, sync::filedumper::StartType},
};
use chin_tools::EResult;

impl ShareAppState {
    pub async fn dump_graph_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        let mapper = &self.mapper;

        mapper
            .dump_to_file::<&PathBuf, GraphMeta>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_graph(&self, endpoint: &crate::krate::sync::po::SyncEndpoint) -> EResult {
        self.sync_one_otid_table1::<GraphMeta>(endpoint).await?;

        Ok(())
    }
}
