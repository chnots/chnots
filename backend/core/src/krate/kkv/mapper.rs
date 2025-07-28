use chin_sql::{OnConflict, str_type::Varchar};
use chin_tools::{AResult, EResult};
use serde::Serialize;

use crate::{MapperType, expand_mt_branch, model::dto::KReq};

use super::*;

pub trait KKVMapper {
    async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp>;
    async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp>;
    async fn kkv_query_many(&self, req: KKVQueryManyReq) -> AResult<KKVQueryManyRsp>;
    async fn kkv_delete(&self, req: KReq<KKVDeleteReq>) -> AResult<KKVDeleteRsp>;
    async fn ensure_table_kkv(&self) -> EResult;

    async fn kkv_transient_query<F, T>(&self, key: &str, mapper: F) -> AResult<Option<T>>
    where
        F: Fn(String) -> AResult<T>,
        T: Send;

    async fn kkv_transisent_overwrite<T: Serialize>(
        &self,
        key: Varchar<500>,
        value: T,
        on_conflict: OnConflict,
    ) -> EResult;
}

impl KKVMapper for MapperType {
    async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp> {
        expand_mt_branch!(self.kkv_overwrite(req))
    }

    async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp> {
        expand_mt_branch!(self.kkv_query(req))
    }

    async fn kkv_delete(&self, req: KReq<super::KKVDeleteReq>) -> AResult<super::KKVDeleteRsp> {
        expand_mt_branch!(self.kkv_delete(req))
    }

    async fn ensure_table_kkv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kkv())
    }

    async fn kkv_query_many(&self, req: KKVQueryManyReq) -> AResult<KKVQueryManyRsp> {
        expand_mt_branch!(self.kkv_query_many(req))
    }

    async fn kkv_transient_query<F, T>(&self, key: &str, mapper: F) -> AResult<Option<T>>
    where
        F: Fn(String) -> AResult<T>,
        T: Send,
    {
        expand_mt_branch!(self.kkv_transient_query(key, mapper))
    }

    async fn kkv_transisent_overwrite<T: Serialize>(
        &self,
        key: Varchar<500>,
        value: T,
        on_conflict: OnConflict,
    ) -> EResult {
        expand_mt_branch!(self.kkv_transisent_overwrite(key, value, on_conflict))
    }
}
