use chin_tools::AResult;

use crate::mapper::{MapperRowType, MapperType};

pub trait Dumper<T> {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        start_tid: chin_sql::time_type::TID,
        page_size: usize,
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
        start_tid: chin_sql::time_type::TID,
        page_size: usize,
        mapper: F,
    ) -> chin_tools::AResult<Vec<E>>
    where
        F: Fn(MapperRowType) -> AResult<E> + Send + Sync + 'static,
        E: Send + 'static,
    {
        match self {
            MapperType::KDb(kdb) => {
                kdb.dump(table_name, start_tid, page_size, move |row| {
                    mapper(MapperRowType::KDb(row))
                })
                .await
            }
        }
    }
}
