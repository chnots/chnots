use std::path::PathBuf;

use anyhow::Ok;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    krate::{
        chnot::{ChnotMetadata, ChnotTag, MdwtRecord},
        sync::{filedumper::StartType, po::SyncEndpoint},
    },
};

impl ShareAppState {
    pub async fn dump_chnot_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        let mapper = &self.mapper;

        mapper
            .dump_to_file::<&PathBuf, ChnotMetadata>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, ChnotTag>(backup_dir, start_type)
            .await?;

        mapper
            .dump_to_file::<&PathBuf, MdwtRecord>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_chnots(&self, endpoint: &SyncEndpoint) -> EResult {
        self.sync_one_otid_table1::<MdwtRecord>(endpoint).await?;
        self.sync_one_otid_table1::<ChnotMetadata>(endpoint).await?;
        self.sync_one_otid_table1::<ChnotTag>(endpoint).await?;
        Ok(())
    }
}
