use chin_sql::{SqlBuilder, SqlDeleter, Wheres, str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};
use chrono::TimeDelta;
use serde::Serialize;

use crate::{
    mapper::db::{
        KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
        helper::create_tables,
    },
    model::{dto::KReq, omit_tid::OmitTID},
};

use super::{
    mapper::{KKVDeserializeMapper, KKVMapper},
    *,
};

impl KKVDeserializeMapper for KDbRow {
    fn to_kkv(self) -> AResult<KKV> {
        let obj = KKV {
            omit_tid: self.try_get(KKV::OMIT_TID)?,
            key: self.try_get(KKV::KEY)?,
            value: self.try_get(KKV::VALUE)?,
            kind: self.try_get(KKV::KIND)?,
            kspace: self.try_get(KKV::KSPACE)?,
            tid: self.try_get(KKV::TID)?,
            archor: self.try_get(KKV::ARCHOR)?,
        };
        Ok(obj)
    }
}

impl KDbExecutor<'_> {
    pub async fn kkv_overwrite(&self, req: KReq<KKVOverwriteReq>) -> AResult<KKVOverwriteRsp> {
        let old = self
            .kkv_query(req.frame(KKVQueryOneReq {
                key: req.key.clone(),
                kind: req.kind.clone(),
            }))
            .await?;
        let old_tid = old.tid.unwrap_or(0.into()).as_utc();
        let now_tid = TID::default();
        let archor = now_tid.as_utc().signed_duration_since(old_tid).abs() > TimeDelta::hours(1);
        let inserter = KKV {
            key: req.key.clone(),
            kind: req.kind.clone(),
            kspace: req.kspace.clone(),
            omit_tid: OmitTID::never(),
            value: req.value.clone(),
            tid: now_tid,
            archor,
        };
        let inserter = inserter
            .to_sql_inserter()
            .on_conflict(chin_sql::OnConflict::Replace(
                [KKV::KEY, KKV::KIND, KKV::KSPACE, KKV::OMIT_TID].join(","),
            ));
        self.exec(inserter).await?;

        Ok(KKVOverwriteRsp {})
    }

    pub async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp> {
        let query = SqlBuilder::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, req.kind.clone()),
            Wheres::equal(KKV::KSPACE, req.kspace.clone()),
            Wheres::equal(KKV::OMIT_TID, OmitTID::never()),
        ]));

        let kv = self.qry_opt(query, KDbRow::to_kkv).await?;

        Ok(KKVQueryOneRsp {
            tid: kv.as_ref().map(|kv| kv.tid),
            value: kv.map(|kv| kv.value),
        })
    }

    pub async fn kkv_transisent_overwrite<T: Serialize>(&self, key: Varchar<500>, value: T) -> EResult {
        self.exec(
            KKVTransient {
                key,
                value: serde_json::to_string(&value)?.into(),
                tid: TID::default(),
            }
            .to_sql_inserter(),
        )
        .await?;
        Ok(())
    }

    pub async fn kkv_transient_query<F, T>(&self, key: &str, mapper: F) -> AResult<Option<T>>
    where
        F: Fn(String) -> AResult<T>,
        T: Send,
    {
        let reader = KKVTransient::pkey_reader(key.to_owned().try_into()?);
        let result = self.qry_opt(reader, Ok).await?;
        if let Some(row) = result {
            Ok(Some(mapper(row.try_get(KKVTransient::VALUE)?)?))
        } else {
            Ok(None)
        }
    }
}

impl KKVMapper for KDb {
    async fn ensure_table_kkv(&self) -> chin_tools::EResult {
        create_tables(vec![KKV::create_sql(), KKVTransient::create_sql()], self).await
    }

    async fn kkv_query_many(&self, req: KKVQueryManyReq) -> AResult<KKVQueryManyRsp> {
        let query = SqlBuilder::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::if_some(req.key, |key| Wheres::equal(KKV::KEY, key)),
            Wheres::if_some(req.kind, |kind| Wheres::equal(KKV::KIND, kind)),
            Wheres::if_some(req.kspace, |kspace| {
                Wheres::equal(KKV::KSPACE, kspace.to_string())
            }),
        ]));

        let kkvs = self.conn().await?.qry_list(query, KDbRow::to_kkv).await?;

        Ok(KKVQueryManyRsp { kkvs })
    }

    async fn kkv_delete(&self, req: KReq<KKVDeleteReq>) -> AResult<KKVDeleteRsp> {
        let del = SqlDeleter::new(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, &req.kind),
            Wheres::equal(KKV::KSPACE, req.kspace.clone()),
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
