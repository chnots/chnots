use std::path::Path;

use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult};

use crate::{
    krate::sync::mapper::Dumper,
    mapper::{MapperRowType, MapperType},
};

pub struct DumpFilenamePattern {
    table_name: String,
    start: TID,
    end: TID,
}

impl DumpFilenamePattern {
    pub fn new(table_name: String, start: TID) -> Self {
        Self {
            table_name,
            start,
            end: TID::default(),
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

pub struct FileDumper {
    table_name: String,
}

impl FileDumper {
    pub async fn dump_one_table<P: AsRef<Path>, F, T>(
        table_name: &str,
        backup_dir: P,
        mapper_type: &MapperType,
        mapper: F,
    ) -> EResult
    where
        F: Fn(MapperRowType) -> AResult<T>,
    {
        let start_tid = DumpFilenamePattern::get_start_time(backup_dir, table_name)?;
        mapper_type.dump(table_name, start_tid, 1000, mapper).await?
    }
}
