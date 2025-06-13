use chin_sql::{SqlDeleter, SqlInserter, SqlReader, Wheres};
use chin_tools::AResult;
use chrono::Local;

use crate::{
    mapper::db::{KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver, KDbRow, KDbRowBehavier},
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

impl KDbExecutor<'_> {
    pub async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp> {
        let inserter = SqlInserter::new(KKV::TABLE)
            .field(KKV::KEY, &req.key)
            .field(KKV::KIND, req.kind)
            .field(KKV::VALUE, &req.value)
            .field(KKV::KSPACE, &req.kspace)
            .field(KKV::INSERT_TIME, Local::now().fixed_offset())
            .on_conflict(chin_sql::OnConflict::Replace([KKV::KEY, KKV::KIND, KKV::KSPACE].join(",")));
        self.exec(inserter).await?;

        Ok(KKVOverwriteRsp {})
    }

    pub async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp> {
        let query = SqlReader::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, req.kind),
            Wheres::equal(KKV::KSPACE, &req.kspace),
        ]));

        let kv = self.qry_opt(query, KDbRow::to_kkv).await?;

        Ok(KKVQueryOneRsp {
            value: kv.map(|kv| kv.value),
        })
    }

}

impl KKVMapper for KDb {
    async fn ensure_table_kkv(&self) -> chin_tools::EResult {
        self.conn()
            .await?
            .create_table(KKV::schema(self.db_type()))
            .await?;
        Ok(())
    }


    async fn kkv_query_many(&self, req: KKVQueryManyReq) -> AResult<KKVQueryManyRsp> {
        let query = SqlReader::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::if_some(req.key, |key| Wheres::equal(KKV::KEY, key)),
            Wheres::if_some(req.kind, |kind| Wheres::equal(KKV::KIND, kind)),
            Wheres::if_some(req.kspace, |kspace| Wheres::equal(KKV::KSPACE, kspace.to_string())),
        ]));

        let kkvs = self.conn().await?.qry_list(query, KDbRow::to_kkv).await?;

        Ok(KKVQueryManyRsp { kkvs })
    }

    async fn kkv_delete(&self, req: KReq<KKVDeleteReq>) -> AResult<KKVDeleteRsp> {
        let del = SqlDeleter::new(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, &req.kind),
            Wheres::equal(KKV::KSPACE, &req.kspace),
        ]));

        self.conn().await?.exec(del).await?;

        Ok(KKVDeleteRsp {})
    }
    
    async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp> {
        self.conn().await?.as_executor().kkv_overwrite(req).await
    }
    
    async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp> {
        self.conn().await?.as_executor().kkv_query(req).await
    }
}
