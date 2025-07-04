use chin_tools::{AResult, EResult};

use crate::{expand_mt_branch, model::dto::KReq, MapperType};

use super::*;

pub trait KKVMapper {
    async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp>;
    async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp>;
    async fn kkv_query_many(&self, req: KKVQueryManyReq) -> AResult<KKVQueryManyRsp>;
    async fn kkv_delete(&self, req: KReq<KKVDeleteReq>) -> AResult<KKVDeleteRsp>;
    async fn ensure_table_kkv(&self) -> EResult;
}

pub(crate) trait KKVDeserializeMapper {
    fn to_kkv(self) -> AResult<KKV>;
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
}
