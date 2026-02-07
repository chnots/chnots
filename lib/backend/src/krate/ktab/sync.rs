use std::path::PathBuf;

use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    krate::{
        ktab::{KTabCellDate, KTabCellDecimal, KTabCellText, KTabMeta},
        sync::filedumper::StartType,
    },
};

impl ShareAppState {
    pub async fn dump_ktab_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        let mapper = &self.mapper;

        mapper
            .dump_to_file::<&PathBuf, KTabMeta>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, KTabCellDate>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, KTabCellDecimal>(backup_dir, start_type)
            .await?;
        mapper
            .dump_to_file::<&PathBuf, KTabCellText>(backup_dir, start_type)
            .await?;

        Ok(())
    }

    pub async fn sync_ktab(&self, endpoint: &crate::krate::sync::po::SyncEndpoint) -> EResult {
        self.sync_one_otid_table_only::<KTabMeta>(endpoint).await?;
        self.sync_one_otid_table_only::<KTabCellDate>(endpoint)
            .await?;
        self.sync_one_otid_table_only::<KTabCellDecimal>(endpoint)
            .await?;
        self.sync_one_otid_table_only::<KTabCellText>(endpoint)
            .await?;

        Ok(())
    }
}
