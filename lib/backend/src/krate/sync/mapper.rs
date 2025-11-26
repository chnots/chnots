use chin_sql::{str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    krate::{
        kkv::{KKVTransient, mapper::KKVMapper},
        sync::{
            dto::{
                SyncDataArg, SyncEndpointCommitRsp, SyncInfo, SyncPageInfo, SyncTIDListArg,
                SyncTIDListPage, SyncTIDListRsp,
            },
            po::{SyncAllEndpoints, SyncLogTransientCommit},
        },
    },
    magics::ALL_ENDPOINTS_KEY,
    mapper::MapperType,
    model::KOtidSupport,
};

pub trait Dumper {
    async fn dump<E>(&self, fetch_data: SyncTIDListPage, hist: bool) -> chin_tools::AResult<Vec<E>>
    where
        E: KOtidSupport;
}

impl Dumper for MapperType {
    async fn dump<E>(&self, fetch_data: SyncTIDListPage, hist: bool) -> chin_tools::AResult<Vec<E>>
    where
        E: KOtidSupport,
    {
        match self {
            MapperType::KDb(kdb) => kdb.dump(fetch_data, hist).await,
        }
    }
}

pub trait SyncMapper {
    async fn ensure_sync_table(&self) -> EResult;
    async fn sync_log_transient_commit(&self, log: SyncLogTransientCommit) -> EResult;
    async fn sync_get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID>;
    async fn sync_tid_list<T: KOtidSupport>(
        &self,
        req: SyncTIDListArg<T>,
    ) -> AResult<SyncTIDListRsp>;
    async fn sync_merge_tids<T: KOtidSupport>(
        &self,
        data: SyncTIDListRsp,
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

    #[allow(dead_code)]
    async fn sync_drop_tmp_table<T: KOtidSupport>(&self, sync_info: &SyncInfo<T>) -> EResult;
}

impl SyncMapper for MapperType {
    async fn ensure_sync_table(&self) -> EResult {
        expand_mt_branch!(self.ensure_sync_table())
    }

    async fn sync_log_transient_commit(&self, log: SyncLogTransientCommit) -> EResult {
        expand_mt_branch!(self.sync_log_transient_commit(log))
    }

    async fn sync_get_sync_time(
        &self,
        table_name: Varchar<100>,
        remote_id: Varchar<100>,
    ) -> AResult<TID> {
        expand_mt_branch!(self.sync_get_sync_time(table_name, remote_id))
    }

    async fn sync_tid_list<T: KOtidSupport>(
        &self,
        req: SyncTIDListArg<T>,
    ) -> AResult<SyncTIDListRsp> {
        expand_mt_branch!(self.sync_tid_list(req))
    }

    async fn sync_merge_tids<T: KOtidSupport>(
        &self,
        data: SyncTIDListRsp,
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
    pub(crate) async fn sync_endpoint_list(&self) -> AResult<SyncAllEndpoints> {
        let kkv: Option<SyncAllEndpoints> = self
            .kkv_transient_fetch::<SyncAllEndpoints>(ALL_ENDPOINTS_KEY)
            .await?;
        match kkv {
            Some(kkv) => Ok(kkv),
            None => Ok(SyncAllEndpoints {
                endpoints: Default::default(),
            }),
        }
    }

    pub(crate) async fn sync_endpoint_commit(
        &self,
        req: SyncAllEndpoints,
    ) -> AResult<SyncEndpointCommitRsp> {
        self.kkv_transisent_commit(
            ALL_ENDPOINTS_KEY.to_string().try_into()?,
            &req,
            chin_sql::OnConflict::Replace(KKVTransient::KEY.to_string()),
        )
        .await?;
        Ok(SyncEndpointCommitRsp {})
    }
}
