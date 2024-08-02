pub mod postgres;

use chin_tools::wrapper::anyhow::{AResult, EResult};
use enum_dispatch::enum_dispatch;
use postgres::Postgres;

use crate::model::{chnot::Chnot, dto::*};

#[enum_dispatch]
pub enum DbType {
    Postgres(Postgres),
}

#[enum_dispatch(DbType)]
pub trait TableFounder {
    /// Main table.
    async fn _ensure_table_chnots(&self) -> EResult;

    // Toent Definations.
    async fn _ensure_table_toent_defi(&self) -> EResult;

    // Toent Instances.
    async fn _ensure_table_toent_inst(&self) -> EResult;

    /// Build all tables
    async fn ensure_tables(&self) -> EResult {
        self._ensure_table_chnots().await?;
        self._ensure_table_toent_defi().await?;
        self._ensure_table_toent_inst().await?;

        Ok(())
    }
}

#[enum_dispatch(DbType)]
pub trait ChnotMapper {
    async fn chnot_overwrite(&self, chnot: Chnot) -> EResult;
    async fn chnot_delete(&self, req: ChnotDeletionReq) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: ChnotQueryReq) -> AResult<ChnotQueryRsp>;
}

#[enum_dispatch(DbType)]
pub trait Db: TableFounder + ChnotMapper {}
