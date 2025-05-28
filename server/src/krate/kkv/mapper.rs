use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    model::dto::KReq,
    MapperType,
};

use super::*;

pub(crate) trait KKVMapper {
    async fn kkv_overwrite_rest(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp>;
    async fn kkv_query_rest(&self, req: KReq<KKVQueryReq>) -> AResult<KKVQueryRsp>;
    async fn kkv_delete_rest(&self, req: KReq<KKVDeleteReq>) -> AResult<KKVDeleteRsp>;
    async fn kkv_overwrite(&self, req: KKVOverwriteReq, extra: KKVReqExtra) -> AResult<KKVOverwriteRsp>;
    async fn kkv_query(&self, req: KKVQueryReq, extra: KKVReqExtra) -> AResult<KKVQueryRsp>;
    async fn kkv_delete(&self, req: KKVDeleteReq, extra: KKVReqExtra) -> AResult<KKVDeleteRsp>;
    async fn ensure_table_kkv(&self) -> EResult;
}

pub(crate) trait KKVDeserializeMapper {
    fn to_kkv(self) -> AResult<KKV>;
}

impl KKVMapper for MapperType {
    async fn kkv_overwrite_rest(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp> {
        expand_mt_branch!(self.kkv_overwrite_rest(req))
    }

    async fn kkv_query_rest(&self, req: KReq<KKVQueryReq>) -> AResult<KKVQueryRsp> {
        expand_mt_branch!(self.kkv_query_rest(req))
    }

    async fn kkv_delete_rest(&self, req: KReq<super::KKVDeleteReq>) -> AResult<super::KKVDeleteRsp> {
        expand_mt_branch!(self.kkv_delete_rest(req))
    }

    async fn ensure_table_kkv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kkv())
    }

    async fn kkv_overwrite(&self, req: KKVOverwriteReq, extra: KKVReqExtra) -> AResult<KKVOverwriteRsp> {
        expand_mt_branch!(self.kkv_overwrite(req, extra))
    }

    async fn kkv_query(&self, req: KKVQueryReq, extra: KKVReqExtra) -> AResult<KKVQueryRsp> {
        expand_mt_branch!(self.kkv_query(req, extra))
    }

    async fn kkv_delete(&self, req: KKVDeleteReq, extra: KKVReqExtra) -> AResult<KKVDeleteRsp> {
        expand_mt_branch!(self.kkv_delete(req, extra))
    }
}

