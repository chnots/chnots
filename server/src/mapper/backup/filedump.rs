use std::{cell::RefCell, fs::{File, OpenOptions}, io::{BufWriter, Write}, path::PathBuf, rc::Rc, time::UNIX_EPOCH};

use anyhow::Context;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::ShareAppState;

use super::TableDumpWriter;

#[derive(Debug, Deserialize, Clone)]
pub struct FileBackupConfig {
    backup_dir: String,
    period: u64,
}

pub enum BackupType {
    Period,
    All,
}

pub struct FileDumpWorker {
    file_prefix: String,
    start_timestamp: DateTime<Utc>,
    writer: Rc<RefCell<BufWriter<File>>>,
    config: FileBackupConfig,
}

impl TableDumpWriter for FileDumpWorker {
    async fn write_one<E: Serialize>(&self, obj: E) -> EResult {
        let obj = serde_json::to_string(&obj);
        if let Ok(obj) = obj {
            let mut w = self.writer.borrow_mut();
            w.write_all("\n".as_bytes())?;
            w.write_all(obj.as_bytes())?;
            w.flush()?;
        } else {
            info!("unable to dump one record.");
        }

        Ok(())
    }
}

impl FileDumpWorker {
    pub async fn new(
        app_state: &ShareAppState,
        file_prefix: &str,
        backup_type: BackupType,
    ) -> AResult<Self> {
        if let Some(config) = app_state.config.file_backup.clone() {
            std::fs::create_dir_all(&config.backup_dir)?;
            let mut read_dir = std::fs::read_dir(config.backup_dir.clone())?;
            let end_time = match backup_type {
                BackupType::Period => {
                    let mut end_time = UNIX_EPOCH.into();
                    while let Some(Ok(next)) = read_dir.next() {
                        let filename = next.file_name().to_string_lossy().to_string();
                        if filename.starts_with(file_prefix) {
                            if let Ok((start, end)) = Self::grab_filename_dt(filename.as_str()) {
                                if end > end_time {
                                    end_time = end;
                                };
                            }
                        }
                    }
                    end_time
                }
                BackupType::All => UNIX_EPOCH.into(),
            };
            let filename = Self::build_file_name(file_prefix, end_time, Utc::now());
            let filepath = PathBuf::from(config.backup_dir.clone()).join(filename);
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(filepath)?;
            let writer = BufWriter::new(file);

            Ok(Self {
                start_timestamp: end_time,
                config,
                file_prefix: file_prefix.to_owned(),
                writer: Rc::new(RefCell::new(writer)),
            })
        } else {
            anyhow::bail!("no file backup config")
        }
    }

    fn grab_filename_dt(filename: &str) -> AResult<(DateTime<Utc>, DateTime<Utc>)> {
        let parts: Vec<&str> = filename.split(|c| c == '.' || c == '-').collect();
        let format = "%Y%m%d%H%M%S";
        let start = parts.get(1).context("unable to get start time")?;
        let end = parts.get(2).context("unable to get end time")?;
        let naive_start = NaiveDateTime::parse_from_str(start, format)?;
        let start_utc = Utc.from_utc_datetime(&naive_start);
        let naive_end = NaiveDateTime::parse_from_str(end, format)?;
        let end_utc = Utc.from_utc_datetime(&naive_end);

        Ok((start_utc, end_utc))
    }

    fn build_file_name(
        table_name: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> String {
        format!(
            "{}-{}-{}.jsonl",
            table_name,
            start_time.format("%Y%m%d%H%M%S"),
            end_time.format("%Y%m%d%H%M%S")
        )
    }
}
