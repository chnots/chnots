pub mod filedump;
pub mod tabledumper;

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

pub trait TableRowCallback {
    async fn callback<E: Serialize>(&self, obj: E) -> EResult;
}

pub enum TableRowCallbackEnum {
    File(FileDumpWorker),
    Network(),
}

impl TableRowCallback for TableRowCallbackEnum {
    async fn callback<E: Serialize>(&self, obj: E) -> EResult {
        match self {
            TableRowCallbackEnum::File(file_dump_worker) => file_dump_worker.callback(obj).await,
            TableRowCallbackEnum::Network() => todo!(),
        }
    }
}
