use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult};
use log::{error, info};
use serde::Deserialize;

use crate::{
    app::ShareAppState,
    config::ShellExpandPath,
    krate::sync::{dto::SyncFetchTIDPage, mapper::Dumper, po::SyncEndpoint},
    mapper::MapperType,
    model::KOtidSupport,
};

#[derive(Debug, Deserialize, Clone)]
pub struct FileBackupConfig {
    pub(crate) backup_dir: ShellExpandPath,
}

pub struct DumpFilenamePattern {
    table_name: String,
    start: TID,
    end: TID,
}

impl DumpFilenamePattern {
    pub fn new(table_name: String, start: TID, end: TID) -> Self {
        Self {
            table_name,
            start,
            end,
        }
    }

    pub fn try_from_file_name(filename: &str) -> AResult<Self> {
        let segs: Vec<&str> = filename.split("-").collect();
        let table_name = segs
            .first()
            .ok_or(anyhow::anyhow!("unable to find table_name, {}", filename))?;
        let start = segs
            .get(1)
            .ok_or(anyhow::anyhow!("unable to find start_time, {}", filename))?;

        let end = segs
            .get(2)
            .ok_or(anyhow::anyhow!("unable to find end_time, {}", filename))?;

        Ok(Self {
            table_name: table_name.to_string(),
            start: start.parse::<i64>()?.into(),
            end: end.parse::<i64>()?.into(),
        })
    }

    pub fn to_file_name(&self) -> String {
        format!(
            "{}-{}-{}-.jsonl",
            self.table_name,
            self.start.as_num(),
            self.end.as_num()
        )
    }

    pub fn get_start_time<P: AsRef<Path>>(backup_dir: P, prefix: &str) -> AResult<TID> {
        let dirs = std::fs::read_dir(backup_dir.as_ref())?;
        let mut start = 0;
        for ele in dirs {
            let entry = ele?;
            let f = entry.file_name();
            let filename = f.to_string_lossy();
            if filename.starts_with(prefix)
                && let Ok(p) = Self::try_from_file_name(filename.as_ref())
                && p.end.as_num() > start
            {
                start = p.end.as_num();
            }
        }

        Ok(start.into())
    }
}

pub struct FileDumper<P: AsRef<Path>, E: KOtidSupport> {
    pub(crate) backup_dir: P,
    pub(crate) end_in: TID,
    pub(crate) start_type: StartType,
    _table_type: PhantomData<E>,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum StartType {
    All,
    #[allow(clippy::upper_case_acronyms)]
    TID(TID),
    Increase,
}

impl MapperType {
    pub(crate) async fn dump_to_file<P: AsRef<Path>, E: KOtidSupport>(
        &self,
        backup_dir: P,
        start_type: StartType,
    ) -> EResult {
        let fd = FileDumper {
            backup_dir,
            end_in: TID::default(),
            start_type,
            _table_type: PhantomData::<E>,
        };

        fd.dump_one_table(self, false).await?;
        fd.dump_one_table(self, true).await?;

        Ok(())
    }
}

impl<P: AsRef<Path>, T: KOtidSupport> FileDumper<P, T> {
    // TODO: rewrite
    pub async fn dump_one_table(&self, mapper_type: &MapperType, hist: bool) -> EResult
    where
        T: KOtidSupport,
    {
        let start_ex = match self.start_type {
            StartType::All => TID::from(0),
            StartType::TID(tid) => tid,
            StartType::Increase => {
                DumpFilenamePattern::get_start_time(&self.backup_dir, T::table_name(hist))?
            }
        };

        let table_name = T::table_name(hist);
        let backup_file = DumpFilenamePattern::new(table_name.to_string(), start_ex, self.end_in);
        let backup_file = PathBuf::new()
            .join(&self.backup_dir)
            .join(backup_file.to_file_name());

        let mut last = start_ex;
        let end = TID::default();
        loop {
            let c = mapper_type
                .dump::<T>(
                    SyncFetchTIDPage::StartEnd {
                        start_ex: last,
                        end_in: end,
                        page_size: 500,
                    },
                    hist,
                )
                .await?;
            let jsonl: Vec<String> = c
                .iter()
                .map(|e| serde_json::to_string(e))
                .collect::<Result<Vec<String>, serde_json::Error>>()?;
            tokio::fs::write(&backup_file, "\n").await?;
            tokio::fs::write(&backup_file, jsonl.join("\n")).await?;

            if c.len() < 500 {
                break;
            } else {
                last = c.iter().map(|c| c.tid()).max().unwrap_or(last);
            }
        }

        Ok(())
    }
}

impl ShareAppState {
    pub async fn dump_to_files(&self, start_type: StartType) -> EResult {
        let Some(backup_dir) = self
            .config
            .file_backup
            .as_ref()
            .map(|c| c.backup_dir.clone())
        else {
            return Ok(());
        };

        let backup_dir: PathBuf = backup_dir.into();
        tokio::fs::create_dir_all(&backup_dir).await?;

        self.dump_chnot_to_file(start_type, &backup_dir).await?;
        self.dump_kfile_to_file(start_type, &backup_dir).await?;
        self.dump_kkv_to_file(start_type, &backup_dir).await?;
        self.dump_kspace_to_file(start_type, &backup_dir).await?;
        self.dump_ktab_to_file(start_type, &backup_dir).await?;
        self.dump_llmchat_to_file(start_type, &backup_dir).await?;

        Ok(())
    }

    async fn sync_to_endpoint_blocking(&self, endpoint: &SyncEndpoint) -> EResult {
        info!("begin to sync with {endpoint:?}");
        self.sync_chnots(endpoint).await?;
        self.sync_kfile(endpoint).await?;
        self.sync_kkv(endpoint).await?;
        self.sync_kspace(endpoint).await?;
        self.sync_ktab(endpoint).await?;
        self.sync_llmchat(endpoint).await?;
        info!("finished to sync with {endpoint:?}");

        Ok(())
    }

    pub async fn sync_to_endpoint(&self, endpoint: &SyncEndpoint) -> EResult {
        let app = self.clone();
        let endpoint = endpoint.clone();
        tokio::spawn(async move {
            let sync_result = app.sync_to_endpoint_blocking(&endpoint).await;
            info!("sync one endpoint result: {sync_result:?}");
        })
        .await?;
        Ok(())
    }

    pub async fn sync_to_all_endpoints(&self) -> EResult {
        for endpoint in &self.get_all_endpoints().await?.endpoints {
            if let Err(err) = self.sync_to_endpoint_blocking(endpoint).await {
                error!(
                    "unable to sync {endpoint:?} -- {err:?}, {:?}",
                    err.backtrace().to_string()
                );
            }
        }

        Ok(())
    }
}
