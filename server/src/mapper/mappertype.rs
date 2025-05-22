use chin_tools::{AResult, EResult};

use crate::krate::{
    chnot::mapper::ChnotMapper, kfile::mapper::KFileMapper, kspace::mapper::WorkspaceMapper,
    ktv::mapper::KTVMapper, llmchat::mapper::LLMChatMapper,
};

use super::{
    db::{postgres::Postgres, sqlite::Sqlite},
    dump::RecordCallbackType,
    DumpMapper, MapperConfig, MapperType,
};

impl Into<AResult<MapperType>> for MapperConfig {
    fn into(self) -> AResult<MapperType> {
        match self {
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
        self.ensure_table_ktv().await?;
        self.ensure_table_workspace().await?;
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
        expand_mt_branch!(self.dump_and_callback(writer))
    }
}
