use chin_sql::{OnConflict, SqlBuilder, SqlDeleter, Wheres, str_type::Varchar, time_type::TID};
use chin_tools::{AResult, EResult};
use chrono::TimeDelta;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDb, KDbBehaiver, KDbExecutor, KDbExecutorBehaiver, KDbRow,
            KDbRowBehavier, helper::create_tables,
        },
    },
    model::dto::KReq,
};

use super::{mapper::KKVMapper, *};

impl TryFrom<&KDbRow> for KKV {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = KKV {
            key: value.try_get(KKV::KEY)?,
            value: value.try_get(KKV::VALUE)?,
            kind: value.try_get(KKV::KIND)?,
            kspace: value.try_get(KKV::KSPACE)?,
            tid: value.try_get(KKV::TID)?,
            archor: value.try_get(KKV::ARCHOR)?,
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
            value: req.value.clone(),
            tid: now_tid,
            archor,
        };
        let inserter = inserter
            .to_sql_inserter()
            .on_conflict(chin_sql::OnConflict::Replace(
                [KKV::KEY, KKV::KIND, KKV::KSPACE].join(","),
            ));
        self.exec(inserter).await?;

        Ok(KKVOverwriteRsp {})
    }

    pub async fn kkv_query(&self, req: KReq<KKVQueryOneReq>) -> AResult<KKVQueryOneRsp> {
        let query = SqlBuilder::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::equal(KKV::KEY, req.key.as_str()),
            Wheres::equal(KKV::KIND, req.kind.clone()),
            Wheres::equal(KKV::KSPACE, req.kspace.clone()),
        ]));

        let kv: Option<KKV> = self.qry_opt(query, |e| (&e).try_into()).await?;

        Ok(KKVQueryOneRsp {
            tid: kv.as_ref().map(|kv| kv.tid),
            value: kv.map(|kv| kv.value),
        })
    }

    pub async fn kkv_transisent_overwrite<T: Serialize>(
        &self,
        key: Varchar<500>,
        value: T,
        on_conflict: OnConflict,
    ) -> EResult {
        self.exec(
            KKVTransient {
                key,
                value: serde_json::to_string(&value)?.into(),
                tid: TID::default(),
            }
            .to_sql_inserter()
            .on_conflict(on_conflict),
        )
        .await?;
        Ok(())
    }

    pub async fn kkv_transient_query<T>(&self, key: &str) -> AResult<Option<T>>
    where
        T: Send + DeserializeOwned,
    {
        let reader = KKVTransient::pkey_reader(key.to_owned().try_into()?);
        let result = self.qry_opt(reader, Ok).await?;
        if let Some(row) = result {
            let s: String = row.try_get(KKVTransient::VALUE)?;
            Ok(Some(serde_json::from_str(&s)?))
        } else {
            Ok(None)
        }
    }
}

impl KKVMapper for KDb {
    async fn ensure_table_kkv(&self) -> chin_tools::EResult {
        create_tables(
            vec![
                KKV::create_sql().to_owned_sql(),
                KKV::hist_table(),
                KKVTransient::create_sql().to_owned_sql(),
            ],
            self,
        )
        .await
    }

    async fn kkv_query_many(&self, req: KKVQueryManyReq) -> AResult<KKVQueryManyRsp> {
        let query = SqlBuilder::read_all(KKV::TABLE).r#where(Wheres::and([
            Wheres::if_some(req.key, |key| Wheres::equal(KKV::KEY, key)),
            Wheres::if_some(req.kind, |kind| Wheres::equal(KKV::KIND, kind)),
            Wheres::if_some(req.kspace, |kspace| {
                Wheres::equal(KKV::KSPACE, kspace.to_string())
            }),
        ]));

        let kkvs = self
            .conn()
            .await?
            .qry_list(query, |e| (&e).try_into())
            .await?;

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

    async fn kkv_transient_query<T>(&self, key: &str) -> AResult<Option<T>>
    where
        T: Send + DeserializeOwned,
    {
        self.conn()
            .await?
            .as_executor()
            .kkv_transient_query(key)
            .await
    }

    async fn kkv_transisent_overwrite<T: Serialize>(
        &self,
        key: Varchar<500>,
        value: T,
        on_conflict: OnConflict,
    ) -> EResult {
        self.conn()
            .await?
            .as_executor()
            .kkv_transisent_overwrite(key, value, on_conflict)
            .await
    }
}

impl Curd for KKV {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.key.clone(), self.kind.clone(), self.kspace.clone())
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
