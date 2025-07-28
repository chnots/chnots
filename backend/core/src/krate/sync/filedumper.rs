use std::path::{Path, PathBuf};

use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult};
use serde::{Deserialize, Serialize};

use crate::{
    app::ShareAppState,
    mapper::{MapperRowType, MapperType},
};

#[derive(Debug, Deserialize, Clone)]
pub struct FileBackupConfig {
    pub(crate) backup_dir: String,
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
            if filename.starts_with(prefix) {
                if let Ok(p) = Self::try_from_file_name(filename.as_ref()) {
                    if p.end.as_num() > start {
                        start = p.end.as_num();
                    }
                }
            }
        }

        Ok(start.into())
    }
}

pub struct FileDumper<P: AsRef<Path>> {
    pub(crate) table_name: String,
    pub(crate) backup_dir: P,
    pub(crate) end_in: TID,
    pub(crate) start_type: StartType,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum StartType {
    All,
    #[allow(clippy::upper_case_acronyms)]
    TID(TID),
    Increase,
}

impl<P: AsRef<Path>> FileDumper<P> {
    // TODO: rewrite
    pub async fn dump_one_table<F, T>(&self, mapper_type: &MapperType, mapper: F) -> EResult
    where
        F: Fn(MapperRowType) -> AResult<T> + Send + Sync + Clone + 'static,
        T: Serialize + Send + 'static,
    {
        let start_ex = match self.start_type {
            StartType::All => TID::from(0),
            StartType::TID(tid) => tid,
            StartType::Increase => {
                DumpFilenamePattern::get_start_time(&self.backup_dir, &self.table_name)?
            }
        };

        let table_name = self.table_name.to_ascii_lowercase();
        let backup_file = DumpFilenamePattern::new(table_name.to_string(), start_ex, self.end_in);
        let backup_file = PathBuf::new()
            .join(&self.backup_dir)
            .join(backup_file.to_file_name());

        // TODO: rewrite
        /*          loop {
            let recs = mapper_type
                .dump(
                    &table_name,
                    super::dto::SyncFetchTIDReq {
                        table_name: "asd",
                        dto: SyncFetchTIDDTO {
                            start_ex,
                            end_in: todo!(),
                            page_size: todo!(),
                            type_table: std::marker::PhantomData,
                            hist: todo!(),
                        },
                    },
                    mapper.clone(),
                )
                .await?;

            if !recs.is_empty() {
                let rec_lines: Result<Vec<String>, serde_json::Error> =
                    recs.iter().map(|e| serde_json::to_string(e)).collect();
                let rec_lines = rec_lines?.iter().join("\n");
                log::info!("write to file {backup_file:?}");
                tokio::fs::write(&backup_file, rec_lines).await?;
            }

            if recs.len() < PAGE_SIZE {
                break;
            }
        }  */

        Ok(())
    }
}

#[macro_export]
macro_rules! dump_table_to_file {
    ($mapper:expr, $table_type:tt, $start_type:expr, $backup_dir:expr, $end_in:expr) => {
        let fd = $crate::krate::sync::filedumper::FileDumper {
            table_name: $table_type::TABLE.to_string(),
            start_type: $start_type.clone(),
            backup_dir: $backup_dir.to_path_buf(),
            end_in: $end_in,
        };

        fd.dump_one_table($mapper, |e| match e {
            $crate::mapper::MapperRowType::KDb(row) => {
                let r: $table_type = (&row).try_into()?;
                Ok(r)
            }
        })
        .await?;
    };
}

impl ShareAppState {
    pub async fn dump_to_files(&self, start_type: StartType) -> EResult {
        self.dump_chnot_to_file(start_type.clone()).await?;
        self.dump_kfile_to_file(start_type.clone()).await?;
        self.dump_kkv_to_file(start_type.clone()).await?;
        self.dump_kspace_to_file(start_type.clone()).await?;
        self.dump_ktab_to_file(start_type.clone()).await?;
        self.dump_llmchat_to_file(start_type.clone()).await?;

        Ok(())
    }

    pub async fn sync_via_network(&self) -> EResult {
        for endpoint in &self.get_all_endpoints().await?.endpoints {
            self.sync_chnots(endpoint).await?;
            self.sync_kfile(endpoint).await?;
            self.sync_kkv(endpoint).await?;
            self.sync_kspace(endpoint).await?;
            self.sync_ktab(endpoint).await?;
            self.sync_llmchat(endpoint).await?;
        }

        Ok(())
    }
}
