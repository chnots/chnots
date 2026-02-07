use std::path::PathBuf;

use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    krate::{kkv::KKV, sync::filedumper::StartType},
};

impl ShareAppState {
    pub async fn dump_kkv_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        let mapper = &self.mapper;
        mapper
            .dump_to_file::<&PathBuf, KKV>(backup_dir, start_type)
            .await?;
        Ok(())
    }

    pub async fn sync_kkv(&self, endpoint: &crate::krate::sync::po::SyncEndpoint) -> EResult {
        self.sync_one_otid_table_only::<KKV>(endpoint).await?;

        Ok(())
    }
}
