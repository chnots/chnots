use anyhow::Context;
use chin_tools::wrapper::anyhow::AResult;
use chrono::Local;

use crate::{
    mapper::{KVDeleteRsp, KVMapper},
    model::dto::{
        kv::{KVOverwriteReq, KVOverwriteRsp, KVQueryReq, KVQueryRsp},
        KReq,
    },
    to_sql,
    util::sql_builder::{PlaceHolderType, SqlSegBuilder, Wheres},
};

use super::DeserializeMapper;

use super::Postgres;

impl KVMapper for Postgres {
    async fn kv_overwrite(
        &self,
        req: KReq<KVOverwriteReq>,
    ) -> chin_tools::wrapper::anyhow::AResult<KVOverwriteRsp> {
        self.client().await?
        .execute(
            "insert into kv(key, content, insert_time) values ($1,$2,$3) on conflict(key) do update set content = $2, update_time = $4",
            &[
                &req.kv.key,
                &req.kv.value,
                &req.kv.insert_time,
                &Local::now().fixed_offset()
            ]
        ).await?;

        Ok(KVOverwriteRsp {})
    }

    async fn kv_query(&self, req: KReq<KVQueryReq>) -> AResult<KVQueryRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from kv")
            .r#where(Wheres::and([Wheres::equal("key", req.key.as_str())]))
            .build(&mut PlaceHolderType::dollar_number())
            .context("Unable to build args")?;

        let row = self
            .client()
            .await?
            .query_opt(&query.seg, to_sql!(query.values))
            .await?;

        let kv = match row {
            Some(row) => Some(Self::to_kv(row)?),
            None => None,
        };

        Ok(KVQueryRsp { kv })
    }

    async fn kv_delete(&self, req: KReq<crate::mapper::KVDeleteReq>) -> AResult<crate::mapper::KVDeleteRsp> {
        self.client().await?
        .execute(
            "delete form kv where key = $1",
            &[
                &req.key,
            ]
        ).await?;

        Ok(KVDeleteRsp {})
    }

    async fn ensure_table_kv(&self) -> chin_tools::wrapper::anyhow::EResult {
        self.create_table(
            "CREATE TABLE IF NOT EXISTS kv (
                key VARCHAR(300) PRIMARY KEY,
                value TEXT NOT NULL,
                update_time TIMESTAMPTZ,
                insert_time TIMESTAMPTZ NOT NULL,
            )",
        )
        .await?;
        Ok(())
    }
}
