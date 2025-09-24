use chin_sql::{OnConflict, str_type::Varchar};
use chin_tools::{AResult, EResult};
use serde::{Serialize, de::DeserializeOwned};

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait KKVMapper {
    async fn kkv_commit(&self, req: KReq<KKVCommitReq>) -> AResult<KKVCommitRsp>;
    async fn kkv_fetch(&self, req: KReq<KKVFetchReq>) -> AResult<KKVFetchRsp>;
    async fn kkv_list(&self, req: KKVListReq) -> AResult<KKVListRsp>;
    async fn kkv_archive(&self, req: KReq<KKVArchiveReq>) -> AResult<KKVArchiveRsp>;
    async fn ensure_table_kkv(&self) -> EResult;

    async fn kkv_transient_fetch<T>(&self, key: &str) -> AResult<Option<T>>
    where
        T: Send + DeserializeOwned;

    async fn kkv_transisent_commit<T: Serialize>(
        &self,
        key: Varchar<500>,
        value: T,
        on_conflict: OnConflict,
    ) -> EResult;
}

impl KKVMapper for MapperType {
    async fn kkv_commit(&self, req: KReq<KKVCommitReq>) -> AResult<KKVCommitRsp> {
        expand_mt_branch!(self.kkv_commit(req))
    }

    async fn kkv_fetch(&self, req: KReq<KKVFetchReq>) -> AResult<KKVFetchRsp> {
        expand_mt_branch!(self.kkv_fetch(req))
    }

    async fn kkv_archive(&self, req: KReq<super::KKVArchiveReq>) -> AResult<super::KKVArchiveRsp> {
        expand_mt_branch!(self.kkv_archive(req))
    }

    async fn ensure_table_kkv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kkv())
    }

    async fn kkv_list(&self, req: KKVListReq) -> AResult<KKVListRsp> {
        expand_mt_branch!(self.kkv_list(req))
    }

    async fn kkv_transient_fetch<T>(&self, key: &str) -> AResult<Option<T>>
    where
        T: Send + DeserializeOwned,
    {
        expand_mt_branch!(self.kkv_transient_fetch(key))
    }

    async fn kkv_transisent_commit<T: Serialize>(
        &self,
        key: Varchar<500>,
        value: T,
        on_conflict: OnConflict,
    ) -> EResult {
        expand_mt_branch!(self.kkv_transisent_commit(key, value, on_conflict))
    }
}
