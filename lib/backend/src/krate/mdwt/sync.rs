use std::path::PathBuf;

use anyhow::Ok;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    krate::{
        mdwt::MdwtRecord,
        sync::{filedumper::StartType, po::SyncEndpoint},
    },
};

impl ShareAppState {
    pub async fn dump_mdwt_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        self.mapper
            .dump_to_file::<&PathBuf, MdwtRecord>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_mdwts(&self, endpoint: &SyncEndpoint) -> EResult {
        self.sync_one_otid_table1::<MdwtRecord>(endpoint).await?;
        Ok(())
    }
}
