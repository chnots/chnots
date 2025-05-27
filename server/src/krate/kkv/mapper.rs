use chin_sql::{SqlDeleter, SqlInserter, SqlReader, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;

use crate::{
    expand_mt_branch,
    mapper::db::{KDb, KDbBehaiver, KDbConnBehaiver, KDbRow},
    model::dto::KReq,
    MapperType,
};

use super::*;

pub(crate) trait KKVMapper {
    async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp>;
    async fn kkv_query(&self, req: KReq<KKVQueryReq>) -> AResult<KKVQueryRsp>;
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

    async fn kkv_query(&self, req: KReq<KKVQueryReq>) -> AResult<KKVQueryRsp> {
        expand_mt_branch!(self.kkv_query(req))
    }

    async fn ensure_table_kkv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kkv())
    }

    async fn kkv_delete(&self, req: KReq<super::KKVDeleteReq>) -> AResult<super::KKVDeleteRsp> {
        expand_mt_branch!(self.kkv_delete(req))
    }
}

impl KKVMapper for KDb {
    async fn kkv_overwrite(
        &self,
        req: KReq<KKVOverwriteReq>,
    ) -> chin_tools::AResult<KKVOverwriteRsp> {
        let inserter = SqlInserter::new(KKV::TABLE)
            .fields(KKV::KEY, &req.key)
            .fields(KKV::KIND, &req.kind)
            .fields(KKV::VALUE, &req.value)
            .fields(KKV::INSERT_TIME, Local::now().fixed_offset());
        self.conn().await?.exec(inserter).await?;

        Ok(KKVOverwriteRsp {})
    }

    async fn kkv_query(&self, req: KReq<KKVQueryReq>) -> AResult<KKVQueryRsp> {
        let query = SqlReader::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, req.kind),
        ]));

        let kv = self
            .conn()
            .await?
            .qry_opt(query, |e| KDbRow::to_kkv(e))
            .await?;

        Ok(KKVQueryRsp {
            value: kv.map(|kv| kv.value),
        })
    }

    async fn kkv_delete(&self, req: KReq<super::KKVDeleteReq>) -> AResult<super::KKVDeleteRsp> {
        let del = SqlDeleter::new(KKV::TABLE).r#where(Wheres::equal(KKV::KEY, &req.key));

        self.conn().await?.exec(del).await?;

        Ok(KKVDeleteRsp {})
    }

    async fn ensure_table_kkv(&self) -> chin_tools::EResult {
        self.create_table(KKV::schema(self.db_type())).await?;
        Ok(())
    }
}
