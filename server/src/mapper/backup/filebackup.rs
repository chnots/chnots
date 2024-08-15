use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use chin_tools::wrapper::anyhow::EResult;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::OpenOptions,
    io::{AsyncWriteExt, BufWriter},
    time,
};
use tracing::{error, info};

use crate::app::ShareAppState;

use super::{BackupTrait, DumpWrapper};

#[derive(Debug, Deserialize, Clone)]
pub struct FileBackupConfig {
    backup_dir: String,
    period: u64,
}

pub struct FileDumpWorker {
    app_state: ShareAppState,
    config: FileBackupConfig,
}

impl FileDumpWorker {
    pub fn new(app_state: ShareAppState, config: FileBackupConfig) -> Self {
        Self { app_state, config }
    }

    pub fn schudele(app_state: &ShareAppState, config: &FileBackupConfig) -> EResult {
        let worker = Self::new(app_state.clone(), config.clone());

        tokio::spawn(async move {
            loop {
                time::sleep(time::Duration::from_secs(worker.config.period)).await;
                worker.dump().await.unwrap();
            }
        });

        Ok(())
    }

    pub async fn dump(&self) -> EResult {
        info!("begin to backup chnots");
        let filename = format!(
            "chnots.{}.jsonl",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        tokio::fs::create_dir_all(self.config.backup_dir.clone()).await?;
        let filepath = PathBuf::from(self.config.backup_dir.clone()).join(filename);

        let _ = self
            .app_state
            .mapper
            .dump_chnots(|chnot| async {
                match Self::dump_one(filepath.as_path(), chnot).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            "unable to write to file: {}, {}",
                            filepath.as_path().to_string_lossy(),
                            err
                        );
                    }
                }
                Ok(())
            })
            .await;

        info!("finished backuping chnots");
        Ok(())
    }

    async fn dump_one<E: Serialize>(filepath: impl AsRef<Path>, obj: DumpWrapper<E>) -> EResult {
        let obj = serde_json::to_string(&obj);

        if let Ok(obj) = obj {
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(filepath)
                .await?;
            let mut writer = BufWriter::new(file);
            writer.write_all("\n".as_bytes()).await?;
            writer.write_all(obj.as_bytes()).await?;
            writer.flush().await?;
        } else {
            info!("unable to dump one record.");
        }

        Ok(())
    }
}
