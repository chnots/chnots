pub mod filebackup;

use std::future::Future;

use chin_tools::wrapper::anyhow::EResult;
use serde::{Deserialize, Serialize};

use crate::model::v1::db::chnot::Chnot;

pub trait BackupTrait {
    async fn dump_chnots<F, R1>(&self, row_writer: F) -> EResult
    where
        F: Fn(DumpWrapper<Chnot>) -> R1,
        R1: Future<Output = EResult>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DumpWrapper<E: Serialize> {
    body: E,
    version: usize,
}

impl<E: Serialize> DumpWrapper<E> {
    pub fn of(body: E, version: usize) -> DumpWrapper<E> {
        Self { body, version }
    }
}
