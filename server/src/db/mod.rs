pub mod postgres;

use chin_tools::wrapper::anyhow::{AResult, EResult};
use enum_dispatch::enum_dispatch;
use postgres::{Postgres, PostgresConfig};
use serde::Deserialize;

use crate::model::dto::*;

#[enum_dispatch]
pub enum MapperType {
    Postgres,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
}

impl Into<AResult<MapperType>> for MapperConfig {
    fn into(self) -> AResult<MapperType> {
        match self {
            MapperConfig::Postgres(config) => {
                let pg = Postgres::new(config)?;
                Ok(MapperType::Postgres(pg))
            }
        }
    }
}

#[enum_dispatch(MapperType)]
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

#[enum_dispatch(MapperType)]
pub trait ChnotMapper {
    async fn chnot_overwrite(&self, chnot: ChnotInsertionReq) -> AResult<ChnotInsertionRsp>;
    async fn chnot_delete(&self, req: ChnotDeletionReq) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: ChnotQueryReq) -> AResult<ChnotQueryRsp>;
}

pub trait Db: TableFounder + ChnotMapper {}
