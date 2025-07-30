use chin_sql::{str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    krate::{
        kkv::{KKVTransient, mapper::KKVMapper},
        sync::{
            dto::{
                SyncAllEndpointsRsp, SyncDataArg, SyncFetchDataPageInfo, SyncFetchTIDArg,
                SyncFetchTIDRsp, SyncInfo, SyncPageInfo,
            },
            po::{SyncAllEndpoints, SyncLogTransient},
        },
    },
    magics::ALL_ENDPOINTS,
    mapper::{MapperRowType, MapperType},
    model::KOtidSupport,
};

pub trait Dumper<T> {
    async fn dump<E, F>(
        &self,
        table_name: &str,
        fetch_data: SyncFetchDataPageInfo,
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
        fetch_data: SyncFetchDataPageInfo,
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
    async fn sync_insert_sync_log(&self, log: SyncLogTransient) -> EResult;
    async fn sync_get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID>;
    async fn sync_fetch_tids<T: KOtidSupport>(
        &self,
        req: SyncFetchTIDArg<T>,
    ) -> AResult<SyncFetchTIDRsp>;
    async fn sync_merge_tids<T: KOtidSupport>(
        &self,
        data: SyncFetchTIDRsp,
        hist: bool,
        sync_info: SyncInfo<T>,
    ) -> EResult;
    async fn sync_fetch_operations<T: KOtidSupport>(
        &self,
        sync_info: &SyncPageInfo<T>,
    ) -> AResult<SyncDataArg<T>>;
    async fn sync_merge_operations<T: KOtidSupport>(
        &self,
        req: SyncDataArg<T>,
    ) -> AResult<SyncDataArg<T>>;
    async fn sync_create_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult;
    async fn sync_drop_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult;
}

impl SyncMapper for MapperType {
    async fn ensure_sync_table(&self) -> EResult {
        expand_mt_branch!(self.ensure_sync_table())
    }

    async fn sync_insert_sync_log(&self, log: SyncLogTransient) -> EResult {
        expand_mt_branch!(self.sync_insert_sync_log(log))
    }

    async fn sync_get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID> {
        expand_mt_branch!(self.sync_get_sync_time(table_name, remote_id))
    }

    async fn sync_fetch_tids<T: KOtidSupport>(
        &self,
        req: SyncFetchTIDArg<T>,
    ) -> AResult<SyncFetchTIDRsp> {
        expand_mt_branch!(self.sync_fetch_tids(req))
    }

    async fn sync_merge_tids<T: KOtidSupport>(
        &self,
        data: SyncFetchTIDRsp,
        hist: bool,
        sync_info: SyncInfo<T>,
    ) -> EResult {
        expand_mt_branch!(self.sync_merge_tids(data, hist, sync_info))
    }

    async fn sync_fetch_operations<T: KOtidSupport>(
        &self,
        sync_info: &SyncPageInfo<T>,
    ) -> AResult<SyncDataArg<T>> {
        expand_mt_branch!(self.sync_fetch_operations(sync_info))
    }

    async fn sync_merge_operations<T: KOtidSupport>(
        &self,
        req: SyncDataArg<T>,
    ) -> AResult<SyncDataArg<T>> {
        expand_mt_branch!(self.sync_merge_operations(req))
    }

    async fn sync_create_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult {
        expand_mt_branch!(self.sync_create_tmp_table(sync_info))
    }

    async fn sync_drop_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult {
        expand_mt_branch!(self.sync_drop_tmp_table(sync_info))
    }
}

impl MapperType {
    pub(crate) async fn get_all_endpoints(&self) -> AResult<SyncAllEndpoints> {
        let kkv: Option<SyncAllEndpoints> = self
            .kkv_transient_query::<SyncAllEndpoints>(ALL_ENDPOINTS)
            .await?;
        match kkv {
            Some(kkv) => Ok(kkv),
            None => Ok(SyncAllEndpoints {
                endpoints: Default::default(),
            }),
        }
    }

    pub(crate) async fn overwrite_endpoints(
        &self,
        req: SyncAllEndpoints,
    ) -> AResult<SyncAllEndpointsRsp> {
        self.kkv_transisent_overwrite(
            ALL_ENDPOINTS.to_string().try_into()?,
            &req,
            chin_sql::OnConflict::Replace(KKVTransient::KEY.to_string()),
        )
        .await?;
        Ok(SyncAllEndpointsRsp {})
    }
}
