use std::path::PathBuf;

use chin_sql::time_type::TID;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    dump_table_to_file, impl_sync_operator,
    krate::{
        kfile::{mapper::KFileMapper, InlineKFile, KFileMeta, QueryInlineKFileReq, QueryInlineKFileRsp},
        sync::{filedumper::StartType, po::SyncEndpoint},
    },
    sync_one,
};

impl ShareAppState {
    pub async fn dump_kfile_to_file(&self, start_type: StartType) -> EResult {
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
        dump_table_to_file!(mapper, KFileMeta, start_type, backup_dir, end_in);
        dump_table_to_file!(mapper, InlineKFile, start_type, backup_dir, end_in);
        Ok(())
    }

    async fn sync_assets(app: &ShareAppState, endpoint: &SyncEndpoint, list: &Vec<KFileMeta>) -> EResult {
        let client = reqwest::Client::builder().build()?;
        for kfm in list {
            if kfm.inline {
                let rsp = client
                    .post(format!("http://{}:{}{}", endpoint.ip, endpoint.port, ""))
                    .json(&QueryInlineKFileReq {
                        sid: Some(kfm.sid.to_string()),
                        meta_id: None,
                        with_omit: Some(true),
                    })
                    .send()
                    .await?
                    .json::<QueryInlineKFileRsp>()
                    .await?;
                for ele in rsp.res {
                    app.mapper.insert_inline_kfile2(ele).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn sync_kfile(&self, endpoint: &SyncEndpoint) -> EResult {
        sync_one!(self, KFileMeta, endpoint, false, Self::sync_assets);
        sync_one!(self, KFileMeta, endpoint, true, Self::sync_assets);

        Ok(())
    }
}

impl_sync_operator! { KFileMeta, id }
