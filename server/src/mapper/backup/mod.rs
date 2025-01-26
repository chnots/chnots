pub mod filedump;
pub mod tabledumpersql;

use chin_tools::wrapper::anyhow::{AResult, EResult};
use filedump::FileDumpWorker;
use serde::{Deserialize, Serialize};
use tabledumpersql::TableDumperSqlBuilder;

pub trait DbBackupTrait {
    type RowType;
    async fn read_iterator<'a, F1, O: Serialize>(
        &self,
        sql_builder: TableDumperSqlBuilder<'a>,
        convert_row_to_obj: F1,
        writer: &TableDumpWriterEnum,
    ) -> EResult
    where
        F1: Fn(Self::RowType) -> AResult<O>;
}

pub trait BackupWorker {
    async fn backup(&self);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DumpWrapper<E: Serialize> {
    body: E,
    version: usize,
    table: String,
}

impl<E: Serialize> DumpWrapper<E> {
    pub fn of(body: E, version: usize, table: &str) -> DumpWrapper<E> {
        Self { body, version, table: table.to_owned() }
    }
}

pub trait TableDumpWriter {
    async fn write_one<E: Serialize>(&self, obj: E) -> EResult;
}

pub enum TableDumpWriterType {
    File,
    Network
}

pub enum TableDumpWriterEnum {
    File(FileDumpWorker),
    Network(),
}

impl TableDumpWriter for TableDumpWriterEnum {
    async fn write_one<E: Serialize>(&self, obj: E) -> EResult {
        match self {
            TableDumpWriterEnum::File(file_dump_worker) => {
                file_dump_worker.write_one(obj).await
            },
            TableDumpWriterEnum::Network() => todo!(),
        }
    }
}
