use anyhow::Context;
use chin_sql::{str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult, utils::id_util::generate_uuid};

use crate::{
    expand_mt_branch,
    krate::{
        kkv::mapper::KKVMapper,
        sync::{dto::FetchDataType, po::SyncLogTransient},
    },
    magics::CLIENT_ID_KEY,
    mapper::{MapperRowType, MapperType},
};

pub trait Dumper<T> {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchDataType,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(T) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static;
}

impl Dumper<MapperRowType> for MapperType {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: FetchDataType,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(MapperRowType) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        match self {
            MapperType::KDb(kdb) => {
                kdb.dump(table_name, fetch_data, move |row| {
                    mapper(MapperRowType::KDb(row))
                })
                .await
            }
        }
    }
}

pub trait SyncMapper {
    async fn ensure_sync_table(&self) -> EResult;
    async fn insert_sync_log(&self, log: SyncLogTransient) -> EResult;
    async fn get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID>;
}

impl SyncMapper for MapperType {
    async fn ensure_sync_table(&self) -> EResult {
        expand_mt_branch!(self.ensure_sync_table())
    }

    async fn insert_sync_log(&self, log: SyncLogTransient) -> EResult {
        expand_mt_branch!(self.insert_sync_log(log))
    }

    async fn get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID> {
        expand_mt_branch!(self.get_sync_time(table_name, remote_id))
    }
}

impl MapperType {
    pub async fn get_instance_id(&self) -> AResult<String> {
        let instance_id = self
            .kkv_transient_query(CLIENT_ID_KEY, Ok)
            .await?
            .context("there is not instance_id in the db")?;
        Ok(instance_id)
    }

    pub async fn init_instance_id(&self) -> EResult {
        let instance_id = self.kkv_transient_query(CLIENT_ID_KEY, Ok).await?;
        if instance_id.is_none() {
            self.kkv_transisent_overwrite(
                CLIENT_ID_KEY.try_into()?,
                generate_uuid(),
                chin_sql::OnConflict::Default,
            )
            .await?;
        }
        Ok(())
    }
}
