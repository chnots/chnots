pub mod filedump;

use chin_tools::wrapper::anyhow::EResult;
use filedump::FileDumpWorker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DumpWrapper<E: Serialize> {
    body: E,
    version: usize,
    table: String,
}

impl<E: Serialize> DumpWrapper<E> {
    pub fn of(body: E, version: usize, table: &str) -> DumpWrapper<E> {
        Self {
            body,
            version,
            table: table.to_owned(),
        }
    }
}

pub trait RowCallback {
    async fn callback<E: Serialize>(&self, obj: E) -> EResult;
}

pub enum RecordCallbackEnum {
    File(FileDumpWorker),
    Network(),
}

impl RowCallback for RecordCallbackEnum {
    async fn callback<E: Serialize>(&self, obj: E) -> EResult {
        match self {
            RecordCallbackEnum::File(file_dump_worker) => file_dump_worker.callback(obj).await,
            RecordCallbackEnum::Network() => todo!(),
        }
    }
}
