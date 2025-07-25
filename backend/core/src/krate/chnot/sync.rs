use std::path::PathBuf;

use chin_sql::time_type::TID;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    dump_table_to_file, impl_sync_operator,
    krate::{
        chnot::{ChnotKindRel, ChnotMetadata, ChnotRecord, ChnotTag},
        sync::{
            dto::{FetchTIDReq, SyncFetchTIDRsp, SyncShakeRspEnum, SyncTableEnum},
            filedumper::StartType,
            mapper::{SyncMapper, SyncOperator},
            po::{SyncEndpoint, SyncLogTransient},
        },
    },
    sync_one,
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

    pub async fn sync_chnots(&self, endpoint: &SyncEndpoint) -> EResult {
        sync_one!(self, ChnotRecord, endpoint, false);
        sync_one!(self, ChnotRecord, endpoint, true);
        sync_one!(self, ChnotMetadata, endpoint, false);
        sync_one!(self, ChnotMetadata, endpoint, true);
        sync_one!(self, ChnotKindRel, endpoint, false);
        sync_one!(self, ChnotKindRel, endpoint, true);
        sync_one!(self, ChnotTag, endpoint, false);
        sync_one!(self, ChnotTag, endpoint, true);

        Ok(())
    }
}

impl_sync_operator! { ChnotMetadata, otid }
impl_sync_operator! { ChnotKindRel, meta_otid }
impl_sync_operator! { ChnotRecord, meta_otid }
impl_sync_operator! { ChnotTag, tag, meta_otid }
