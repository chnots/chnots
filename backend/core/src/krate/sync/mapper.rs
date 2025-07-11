use chin_tools::AResult;

use crate::{
    krate::sync::dto::FetchDataType,
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
