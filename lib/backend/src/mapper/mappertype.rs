use anyhow::Context as _;
use chin_sql::OnConflict;
use chin_tools::{AResult, EResult, utils::id_util::generate_uuid};
use log::info;

use crate::{
    krate::{
        chnot::mapper::ChnotMapper, kfile::mapper::KFileMapper, kkv::mapper::KKVMapper,
        kspace::mapper::KSpaceMapper, ktab::mapper::KTabMapper, llmchat::mapper::LLMChatMapper,
        mdwt::mapper::MdwtMapper, sync::mapper::SyncMapper,
    },
    magics::CLIENT_ID_KEY,
    mapper::{MapperConfig, MapperType},
};

use super::db::{postgres::Postgres, sqlite::Sqlite};

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
        self.ensure_table_mdwt().await?;
        self.ensure_table_chnot().await?;
        self.ensure_table_kfile().await?;
        self.ensure_table_kkv().await?;
        self.ensure_table_llm_chat().await?;
        self.kspace_ensure_data().await?;
        self.ensure_ktab_tables().await?;
        self.ensure_sync_table().await?;
        self.commit_chnot_meta_value(CLIENT_ID_KEY, generate_uuid(), chin_sql::OnConflict::Ignore)
            .await?;

        Ok(())
    }

    pub async fn fetch_chnot_meta_value(&self, key: &'static str) -> AResult<String> {
        let instance_id: String = self
            .kkv_transient_fetch(key)
            .await?
            .context("there is not value in the db")?;
        info!("instance id: {instance_id}");
        Ok(instance_id)
    }

    pub async fn commit_chnot_meta_value(
        &self,
        key: &'static str,
        value: String,
        on_conflict: OnConflict,
    ) -> EResult {
        let instance_id: Option<String> = self.kkv_transient_fetch(CLIENT_ID_KEY).await?;
        if instance_id.is_none() {
            self.kkv_transisent_commit(key.try_into()?, value, on_conflict)
                .await?;
        }
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
