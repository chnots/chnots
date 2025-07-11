use std::path::PathBuf;

use chin_sql::time_type::TID;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    dump_table_to_file,
    krate::{kkv::KKV, sync::filedumper::StartType},
};

impl ShareAppState {
    pub async fn dump_kkv_to_file(&self, start_type: StartType) -> EResult {
        let mapper = &self.mapper;
        let Some(backup_dir) = self
            .config
            .file_backup
            .as_ref()
            .map(|c| c.backup_dir.clone())
        else {
            return Ok(());
        };

        let backup_dir: PathBuf = backup_dir.into();
        let end_in = TID::default();
        dump_table_to_file!(mapper, KKV, start_type, backup_dir, end_in);
        Ok(())
    }
}
