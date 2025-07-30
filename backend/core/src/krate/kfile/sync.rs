use std::{ops::Deref, path::PathBuf};

use chin_sql::time_type::TID;
use chin_tools::EResult;

use crate::{
    app::ShareAppState,
    dump_table_to_file,
    krate::{
        kfile::{
            InlineKFile, KFILE_INLINE_INSERT2, KFileInlineInsert2Req, KFileMeta,
            QueryInlineKFileReq, QueryInlineKFileRsp, mapper::KFileMapper,
        },
        sync::{
            dto::SyncDataArg, filedumper::StartType, networksync::OtidRelatedWorker,
            po::SyncEndpoint,
        },
    },
};

struct KFileAssetWorker {
    app: ShareAppState,
}

struct KFileMetaAndHist<'a> {
    file: &'a KFileMeta,
    hist: bool,
}
impl<'a> Deref for KFileMetaAndHist<'a> {
    type Target = &'a KFileMeta;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl KFileAssetWorker {
    async fn pull_kfile(
        &self,
        endpoint: &SyncEndpoint,
        list: Vec<KFileMetaAndHist<'_>>,
    ) -> EResult {
        let client = reqwest::Client::builder().build()?;
        for kfm in list {
            if kfm.inline {
                let rsp = client
                    .get(endpoint.to_url(crate::krate::kfile::dto::KFILE_INLINE_GET_BY_SID))
                    .query(&QueryInlineKFileReq {
                        sid: Some(kfm.sid.clone()),
                        meta_id: None,
                        with_omit: Some(true),
                    })
                    .send()
                    .await?
                    .json::<QueryInlineKFileRsp>()
                    .await?;
                for ele in rsp.res {
                    self.app.insert_inline_kfile2(ele).await?;
                }
            }
        }

        Ok(())
    }

    async fn push_kfile(
        &self,
        endpoint: &SyncEndpoint,
        list: Vec<KFileMetaAndHist<'_>>,
    ) -> EResult {
        let client = reqwest::Client::builder().build()?;

        for kfm in list {
            if kfm.inline {
                let rsp = self.app.query_inline_kfile_by_sid(kfm.sid.clone()).await?;
                if let Some(c) = rsp.file {
                    client
                        .put(endpoint.to_url(KFILE_INLINE_INSERT2))
                        .json(&KFileInlineInsert2Req { file: c })
                        .send()
                        .await?;
                }
            }
        }

        Ok(())
    }
}

impl OtidRelatedWorker<KFileMeta> for KFileAssetWorker {
    async fn before_send(&self, endpoint: &SyncEndpoint, arg: &SyncDataArg<KFileMeta>) -> EResult {
        for ele in &arg.cmds {
            if let crate::krate::sync::dto::SyncDataOperation::Push { data, hist } = ele {
                self.push_kfile(
                    endpoint,
                    vec![KFileMetaAndHist {
                        file: data,
                        hist: *hist,
                    }],
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn before_merge(&self, endpoint: &SyncEndpoint, arg: &SyncDataArg<KFileMeta>) -> EResult {
        for ele in &arg.cmds {
            if let crate::krate::sync::dto::SyncDataOperation::Push { data, hist } = ele {
                self.pull_kfile(
                    endpoint,
                    vec![KFileMetaAndHist {
                        file: data,
                        hist: *hist,
                    }],
                )
                .await?;
            }
        }

        Ok(())
    }
}

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

    pub async fn sync_kfile(&self, endpoint: &SyncEndpoint) -> EResult {
        let worker = KFileAssetWorker { app: self.clone() };
        self.sync_one_otid_table(std::marker::PhantomData::<KFileMeta>, endpoint, &worker)
            .await?;

        Ok(())
    }
}
