use super::{
    sql::{SqlReader, Wheres},
    KDb, KDbBehaiver, KDbConnBehaiver, KDbRow,
};
use chin_sql::{SqlDeleter, SqlInserter};
use chin_tools::AResult;

use crate::{
    mapper::{DeserializeMapper, KVDeleteRsp, KVMapper},
    model::{
        db::kv::KV,
        dto::{
            kv::{KVOverwriteReq, KVOverwriteRsp, KVQueryReq, KVQueryRsp},
            KReq,
        },
    },
};

impl KVMapper for KDb {
    async fn kv_overwrite(
        &self,
        req: KReq<KVOverwriteReq>,
    ) -> chin_tools::wrapper::anyhow::AResult<KVOverwriteRsp> {
        let kv = &req.kv;
        let inserter = SqlInserter::new(KV::TABLE)
            .fields(KV::KEY, &kv.key)
            .fields(KV::VALUE, &kv.value)
            .fields(KV::INSERT_TIME, &kv.insert_time);
        self.conn().await?.exec(inserter).await?;

        Ok(KVOverwriteRsp {})
    }

    async fn kv_query(&self, req: KReq<KVQueryReq>) -> AResult<KVQueryRsp> {
        let query = SqlReader::read_all(KV::TABLE).r#where(Wheres::and([Wheres::equal(
            KV::KEY,
            req.key.as_str(),
        )]));

        let kv = self
            .conn()
            .await?
            .qry_opt(query, |e| KDbRow::to_kv(e))
            .await?;

        Ok(KVQueryRsp { kv })
    }

    async fn kv_delete(
        &self,
        req: KReq<crate::mapper::KVDeleteReq>,
    ) -> AResult<crate::mapper::KVDeleteRsp> {
        let del =
            SqlDeleter::new(KV::TABLE).r#where(Wheres::equal(KV::KEY, &req.key));

        self.conn().await?.exec(del).await?;

        Ok(KVDeleteRsp {})
    }

    async fn ensure_table_kv(&self) -> chin_tools::wrapper::anyhow::EResult {
        self.create_table(KV::schema(self.db_type()))
            .await?;
        Ok(())
    }
}
