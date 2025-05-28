use chin_sql::OnConflict;
use chin_tools::{AResult, EResult};

use crate::krate::{
    chnot::mapper::ChnotMapper, kfile::mapper::KFileMapper, kkv::mapper::KKVMapper,
    kspace::mapper::KSpaceMapper, llmchat::mapper::LLMChatMapper,
};

use super::{
    db::{postgres::Postgres, sqlite::Sqlite},
    dump::RecordCallbackType,
    DumpMapper, MapperConfig, MapperType,
};

impl TryFrom<MapperConfig> for MapperType {
    type Error = anyhow::Error;

    fn try_from(value: MapperConfig) -> Result<Self, Self::Error> {
        match value {
            MapperConfig::Postgres(config) => {
                let pg = Postgres::new(config)?;
                Ok(MapperType::KDb(super::db::KDb::Postgres(pg)))
            }
            MapperConfig::Sqlite(config) => {
                let sqlite = Sqlite::new(config)?;
                Ok(MapperType::KDb(super::db::KDb::Sqlite(sqlite)))
            }
        }
    }
}

impl MapperType {
    pub(crate) async fn ensure_tables(&self) -> EResult {
        self.ensure_table_chnot().await?;
        self.ensure_table_kfile().await?;
        self.ensure_table_kkv().await?;
        self.ensure_table_llm_chat().await?;
        Ok(())
    }
}

#[macro_export]
macro_rules! expand_mt_branch {
    ($self:ident.$method:ident($($arg:expr),*)) => {
        match $self {
            MapperType::KDb(db) => db.$method($($arg),*).await,
        }
    };
}

impl MapperType {
    pub(crate) async fn dump_and_callback(&self, writer: &RecordCallbackType) -> EResult {
        match self {
            MapperType::KDb(kdb) => kdb.dump_and_callback(writer).await,
        }
    }
}

pub(crate) trait InserterBehavier<T> {
    async fn insert(&self, t: T, on_conflict: OnConflict) -> AResult<usize>;
}
