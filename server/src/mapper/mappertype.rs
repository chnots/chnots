use chin_tools::EResult;

use crate::krate::{
    chnot::mapper::ChnotMapper, kfile::mapper::KFileMapper, kkv::mapper::KKVMapper,
    kspace::mapper::KSpaceMapper, ktab::mapper::KTabMapper, llmchat::mapper::LLMChatMapper,
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
        self.kspace_ensure_data().await?;
        self.ensure_ktab_tables().await?;
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
