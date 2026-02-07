use chin_sql::{str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    krate::{
        kkv::{KKVTransient, mapper::KKVMapper},
        sync::{
            dto::{
                SyncDataDto, SyncEndpointCommitRsp, SyncInfo, SyncOtidTIDListRsp, SyncPageInfo,
                SyncTIDListArg, SyncTIDListPage,
            },
            po::{SyncAllEndpoints, SyncLogTransientCommit},
        },
    },
    magics::ALL_ENDPOINTS_KEY,
    mapper::MapperType,
    model::{OtidTableSupport, SidTableSupport},
};

pub trait Dumper {
    async fn dump<E>(&self, fetch_data: SyncTIDListPage, hist: bool) -> chin_tools::AResult<Vec<E>>
    where
        E: OtidTableSupport;
}

impl Dumper for MapperType {
    async fn dump<E>(&self, fetch_data: SyncTIDListPage, hist: bool) -> chin_tools::AResult<Vec<E>>
    where
        E: OtidTableSupport,
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
    async fn sync_otid_tid_list<T: OtidTableSupport>(
        &self,
        req: SyncTIDListArg<T>,
    ) -> AResult<SyncOtidTIDListRsp>;
    async fn sync_otid_merge_tids<T: OtidTableSupport>(
        &self,
        data: SyncOtidTIDListRsp,
        hist: bool,
        sync_info: SyncInfo<T>,
    ) -> EResult;
    async fn sync_fetch_operations<T: OtidTableSupport>(
        &self,
        sync_info: &SyncPageInfo<T>,
    ) -> AResult<SyncDataDto<T>>;
    async fn sync_otid_merge_operations<T: OtidTableSupport>(
        &self,
        req: SyncDataDto<T>,
    ) -> AResult<SyncDataDto<T>>;
    async fn sync_otid_create_tmp_table<T: OtidTableSupport>(
        &self,
        sync_info: &SyncInfo<T>,
    ) -> EResult;

    async fn sync_otid_drop_tmp_table<T: OtidTableSupport>(
        &self,
        sync_info: &SyncInfo<T>,
    ) -> EResult;

    async fn po_sid_sync_list<const LIMIT: usize, T: SidTableSupport>(
        &self,
        sync_info: Vec<Varchar<LIMIT>>,
    ) -> AResult<Vec<T>>;

    async fn po_sid_sync_commit<T: SidTableSupport>(&self, sync_info: Vec<T>) -> EResult;
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

    async fn sync_otid_tid_list<T: OtidTableSupport>(
        &self,
        req: SyncTIDListArg<T>,
    ) -> AResult<SyncOtidTIDListRsp> {
        expand_mt_branch!(self.sync_otid_tid_list(req))
    }

    async fn sync_otid_merge_tids<T: OtidTableSupport>(
        &self,
        data: SyncOtidTIDListRsp,
        hist: bool,
        sync_info: SyncInfo<T>,
    ) -> EResult {
        expand_mt_branch!(self.sync_otid_merge_tids(data, hist, sync_info))
    }

    async fn sync_fetch_operations<T: OtidTableSupport>(
        &self,
        sync_info: &SyncPageInfo<T>,
    ) -> AResult<SyncDataDto<T>> {
        expand_mt_branch!(self.sync_fetch_operations(sync_info))
    }

    async fn sync_otid_merge_operations<T: OtidTableSupport>(
        &self,
        req: SyncDataDto<T>,
    ) -> AResult<SyncDataDto<T>> {
        expand_mt_branch!(self.sync_otid_merge_operations(req))
    }

    async fn sync_otid_create_tmp_table<T: OtidTableSupport>(
        &self,
        sync_info: &SyncInfo<T>,
    ) -> EResult {
        expand_mt_branch!(self.sync_otid_create_tmp_table(sync_info))
    }

    async fn sync_otid_drop_tmp_table<T: OtidTableSupport>(
        &self,
        sync_info: &SyncInfo<T>,
    ) -> EResult {
        expand_mt_branch!(self.sync_otid_drop_tmp_table(sync_info))
    }

    async fn po_sid_sync_list<const LIMIT: usize, T: SidTableSupport>(
        &self,
        sync_info: Vec<Varchar<LIMIT>>,
    ) -> AResult<Vec<T>> {
        expand_mt_branch!(self.po_sid_sync_list(sync_info))
    }

    async fn po_sid_sync_commit<T: SidTableSupport>(&self, sync_info: Vec<T>) -> EResult {
        expand_mt_branch!(self.po_sid_sync_commit(sync_info))
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
