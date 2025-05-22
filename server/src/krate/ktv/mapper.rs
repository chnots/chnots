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

pub(crate) trait KTVMapper {
    async fn ktv_overwrite(&self, req: KReq<KTVOverwriteReq>) -> AResult<KTVOverwriteRsp>;
    async fn ktv_query(&self, req: KReq<KTVQueryReq>) -> AResult<KTVQueryRsp>;
    async fn ktv_delete(&self, req: KReq<KTVDeleteReq>) -> AResult<KTVDeleteRsp>;
    async fn ensure_table_ktv(&self) -> EResult;
}

pub(crate) trait KTVDeserializeMapper {
    fn to_ktv(self) -> AResult<KTV>;
}


impl KTVMapper for MapperType {
    async fn ktv_overwrite(&self, req: KReq<KTVOverwriteReq>) -> AResult<KTVOverwriteRsp> {
        expand_mt_branch!(self.ktv_overwrite(req))
    }

    async fn ktv_query(&self, req: KReq<KTVQueryReq>) -> AResult<KTVQueryRsp> {
        expand_mt_branch!(self.ktv_query(req))
    }

    async fn ensure_table_ktv(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_ktv())
    }

    async fn ktv_delete(&self, req: KReq<super::KTVDeleteReq>) -> AResult<super::KTVDeleteRsp> {
        expand_mt_branch!(self.ktv_delete(req))
    }
}

impl KTVMapper for KDb {
    async fn ktv_overwrite(
        &self,
        req: KReq<KTVOverwriteReq>,
    ) -> chin_tools::AResult<KTVOverwriteRsp> {
        let inserter = SqlInserter::new(KTV::TABLE)
            .fields(KTV::KEY, &req.key)
            .fields(KTV::TTYPE, &req.ttype)
            .fields(KTV::VALUE, &req.value)
            .fields(KTV::INSERT_TIME, Local::now().fixed_offset());
        self.conn().await?.exec(inserter).await?;

        Ok(KTVOverwriteRsp {})
    }

    async fn ktv_query(&self, req: KReq<KTVQueryReq>) -> AResult<KTVQueryRsp> {
        let query = SqlReader::read_all(KTV::TABLE).r#where(Wheres::and([
            Wheres::equal(KTV::KEY, req.key.as_str()),
            Wheres::equal(KTV::TTYPE, req.ttype),
        ]));

        let kv = self
            .conn()
            .await?
            .qry_opt(query, |e| KDbRow::to_ktv(e))
            .await?;

        Ok(KTVQueryRsp {
            value: kv.map(|kv| kv.value),
        })
    }

    async fn ktv_delete(&self, req: KReq<super::KTVDeleteReq>) -> AResult<super::KTVDeleteRsp> {
        let del = SqlDeleter::new(KTV::TABLE).r#where(Wheres::equal(KTV::KEY, &req.key));

        self.conn().await?.exec(del).await?;

        Ok(KTVDeleteRsp {})
    }

    async fn ensure_table_ktv(&self) -> chin_tools::EResult {
        self.create_table(KTV::schema(self.db_type())).await?;
        Ok(())
    }
}
