use anyhow::Context;
use chin_tools::{AResult, EResult, utils::id_util::generate_uuid};

use crate::{
    krate::{kkv::mapper::KKVMapper, sync::dto::FetchDataType},
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
