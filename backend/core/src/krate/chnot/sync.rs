use std::path::PathBuf;

use chin_sql::time_type::TID;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    dump_table_to_file,
    krate::{
        chnot::{ChnotKindRel, ChnotMetadata, ChnotRecord, ChnotTag},
        sync::filedumper::StartType,
    },
};

impl ShareAppState {
    pub async fn dump_chnot_to_file(&self, start_type: StartType) -> EResult {
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
        dump_table_to_file!(mapper, ChnotMetadata, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, ChnotTag, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, ChnotKindRel, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, ChnotRecord, start_type, backup_dir, end_in);

        Ok(())
    }
}
