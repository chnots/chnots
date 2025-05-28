use chin_sql::{SqlDeleter, SqlInserter, SqlReader, Wheres};
use chin_tools::AResult;
use chrono::Local;

use crate::{
    magics::kspace_name,
    mapper::db::{
        KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    },
    model::dto::KReq,
};

use super::{
    mapper::{KKVDeserializeMapper, KKVMapper},
    *,
};

impl KKVDeserializeMapper for KDbRow {
    fn to_kkv(self) -> AResult<KKV> {
        let obj = KKV {
            insert_time: self.try_get(KKV::INSERT_TIME)?,
            key: self.try_get(KKV::KEY)?,
            value: self.try_get(KKV::VALUE)?,
            kind: self.try_get(KKV::KIND)?,
            update_time: self.try_get(KKV::UPDATE_TIME)?,
            kspace: self.try_get(KKV::KSPACE)?,
        };
        Ok(obj)
    }
}

impl KKVMapper for KDb {
    async fn kkv_overwrite_rest(
        &self,
        req: KReq<KKVOverwriteReq>,
    ) -> chin_tools::AResult<KKVOverwriteRsp> {
        self.kkv_overwrite(
            req.body,
            KKVReqExtra {
                kspace: Some(req.kspace),
            },
        )
        .await
    }

    async fn kkv_query_rest(&self, req: KReq<KKVQueryReq>) -> AResult<KKVQueryRsp> {
        self.kkv_query(
            req.body,
            KKVReqExtra {
                kspace: Some(req.kspace),
            },
        )
        .await
    }

    async fn kkv_delete_rest(
        &self,
        req: KReq<super::KKVDeleteReq>,
    ) -> AResult<super::KKVDeleteRsp> {
        self.kkv_delete(
            req.body,
            KKVReqExtra {
                kspace: Some(req.kspace),
            },
        )
        .await
    }

    async fn ensure_table_kkv(&self) -> chin_tools::EResult {
        self.conn()
            .await?
            .create_table(KKV::schema(self.db_type()))
            .await?;
        Ok(())
    }

    async fn kkv_overwrite(
        &self,
        req: KKVOverwriteReq,
        extra: KKVReqExtra,
    ) -> AResult<KKVOverwriteRsp> {
        let inserter = SqlInserter::new(KKV::TABLE)
            .field(KKV::KEY, &req.key)
            .field(KKV::KIND, &req.kind)
            .field(KKV::VALUE, &req.value)
            .field(KKV::KSPACE, kspace_name(extra.kspace.as_ref()))
            .field(KKV::INSERT_TIME, Local::now().fixed_offset());
        self.conn().await?.exec(inserter).await?;

        Ok(KKVOverwriteRsp {})
    }

    async fn kkv_query(&self, req: KKVQueryReq, extra: KKVReqExtra) -> AResult<KKVQueryRsp> {
        let query = SqlReader::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, req.kind),
            Wheres::equal(KKV::KSPACE, kspace_name(extra.kspace.as_ref())),
        ]));

        let kv = self.conn().await?.qry_opt(query, KDbRow::to_kkv).await?;

        Ok(KKVQueryRsp {
            value: kv.map(|kv| kv.value),
        })
    }

    async fn kkv_delete(&self, req: KKVDeleteReq, extra: KKVReqExtra) -> AResult<KKVDeleteRsp> {
        let del = SqlDeleter::new(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, req.kind),
            Wheres::equal(KKV::KSPACE, kspace_name(extra.kspace.as_ref())),
        ]));

        self.conn().await?.exec(del).await?;

        Ok(KKVDeleteRsp {})
    }
}
