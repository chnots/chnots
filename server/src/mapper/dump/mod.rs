pub(crate) mod filedump;

use chin_tools::EResult;
use filedump::FileDumpWorker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DumpWrapper<E: Serialize> {
    body: E,
    version: usize,
    table: String,
}

impl<E: Serialize> DumpWrapper<E> {
    pub(crate) fn of(body: E, version: usize, table: &str) -> DumpWrapper<E> {
        Self {
            body,
            version,
            table: table.to_owned(),
        }
    }
}

pub(crate) trait RowCallback {
    async fn callback<E: Serialize>(&self, obj: E) -> EResult;
}

pub(crate) enum RecordCallbackType {
    File(FileDumpWorker),
    Network(),
}

impl RowCallback for RecordCallbackType {
    async fn callback<E: Serialize>(&self, obj: E) -> EResult {
        match self {
            RecordCallbackType::File(file_dump_worker) => file_dump_worker.callback(obj).await,
            RecordCallbackType::Network() => todo!(),
        }
    }
}
