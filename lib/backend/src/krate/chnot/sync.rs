use std::path::PathBuf;

use anyhow::Ok;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    krate::{
        chnot::{ChnotMeta, ChnotThreadMeta, ChnotThreadOrder},
        sync::{filedumper::StartType, po::SyncEndpoint},
    },
};

impl ShareAppState {
    pub async fn dump_chnot_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        let mapper = &self.mapper;

        mapper
            .dump_to_file::<&PathBuf, ChnotThreadMeta>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_chnots(&self, endpoint: &SyncEndpoint) -> EResult {
        self.sync_one_otid_table_only::<ChnotThreadMeta>(endpoint)
            .await?;
        self.sync_one_otid_table_only::<ChnotMeta>(endpoint).await?;
        self.sync_one_otid_table_only::<ChnotThreadOrder>(endpoint)
            .await?;
        Ok(())
    }
}
