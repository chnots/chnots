use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::{DateTime, FixedOffset, TimeDelta};

use super::{DeserializeMapper, KDb, KDbBehaiver, KDbConnBehaiver, KDbRow};
use crate::{
    mapper::{KVMapper, ResourceMapper},
    model::{
        db::resource::*,
        dto::{resource::*, KReq},
    },
};

use chin_sql::{LimitOffset, OnConflict, SqlDeleter, SqlInserter, SqlReader, Wheres};

impl ResourceMapper for KDb {
    async fn ensure_table_resource(&self) -> EResult {
        self.create_table(Resource::schema(self.db_type())).await
    }

    async fn ensure_table_inline_resource(&self) -> EResult {
        self.create_table(InlineResource::schema(self.db_type()))
            .await
    }

    async fn insert_resource(&self, res: &Resource) -> AResult<Resource> {
        let Resource {
            ori_filename,
            id,
            content_type,
            namespace,
            delete_time: _,
            insert_time: _,
        } = res;

        let conn = self.conn().await?;

        let insert_time = chrono::Utc::now().to_owned().fixed_offset();

        conn.exec(
            SqlInserter::new(Resource::TABLE)
                .fields(Resource::ID, id.to_owned())
                .fields(Resource::ORI_FILENAME, ori_filename.to_owned())
                .fields(Resource::NAMESPACE, namespace.to_owned())
                .fields(Resource::CONTENT_TYPE, content_type.to_owned())
                .fields(Resource::INSERT_TIME, insert_time.to_owned()),
        )
        .await
        .map(|_| Resource {
            id: id.to_owned(),
            namespace: namespace.to_owned(),
            ori_filename: ori_filename.to_string(),
            content_type: content_type.to_owned(),
            insert_time: insert_time.fixed_offset(),
            delete_time: None,
        })
    }

    async fn query_resource_by_id(&self, id: &str) -> AResult<Resource> {
        let conn = self.conn().await?;
        let res = conn
            .qry_one(
                SqlReader::read_all(Resource::TABLE).r#where(Wheres::equal(Resource::ID, id)),
                |e| e.to_resource(),
                false,
            )
            .await?;
        Ok(res)
    }

    async fn insert_inline_resource(
        &self,
        req: &KReq<InsertInlineResourceReq>,
    ) -> anyhow::Result<InsertInlineResourceRsp> {
        let delete_sql = SqlDeleter::new(InlineResource::TABLE).r#where(Wheres::and([
            Wheres::equal(InlineResource::RID, &req.res.rid),
            Wheres::equal(InlineResource::ARCHOR, false),
        ]));

        let last_archor_sql =
            SqlReader::read(InlineResource::TABLE, &[InlineResource::INSERT_TIME])
                .r#where(Wheres::and([
                    Wheres::equal(InlineResource::RID, &req.res.rid),
                    Wheres::equal(InlineResource::ARCHOR, true),
                ]))
                .limit(1);

        self.conn().await?.exec(delete_sql).await?;
        let last_archor: Option<DateTime<FixedOffset>> = self
            .conn()
            .await?
            .qry_opt(last_archor_sql, |e| {
                e.try_get_df(InlineResource::INSERT_TIME)
            })
            .await?;

        let archorp = match last_archor {
            Some(last) => {
                req.res.insert_time.signed_duration_since(last)
                    > TimeDelta::seconds(req.archor_intervals)
            }
            None => true,
        };

        self.conn()
            .await?
            .exec(
                SqlInserter::new(InlineResource::TABLE)
                    .fields(InlineResource::ID, &req.res.id)
                    .fields(InlineResource::RID, &req.res.rid)
                    .fields(InlineResource::NAME, &req.res.name)
                    .fields(InlineResource::CONTENT, &req.res.content)
                    .fields(InlineResource::CONTENT_TYPE, &req.res.content_type)
                    .fields(InlineResource::INSERT_TIME, &req.res.insert_time)
                    .fields(InlineResource::NAMESPACE, &req.res.namespace)
                    .fields(InlineResource::ARCHOR, archorp)
                    .on_conflict({
                        match req.ignore_conflict.as_ref() {
                            Some(ic) => {
                                if *ic {
                                    OnConflict::Ignore
                                } else {
                                    OnConflict::Default
                                }
                            }
                            None => OnConflict::Default,
                        }
                    }),
            )
            .await?;

        Ok(InsertInlineResourceRsp {})
    }

    async fn query_inline_resource(
        &self,
        req: KReq<QueryInlineResourceReq>,
    ) -> anyhow::Result<QueryInlineResourceRsp> {
        let query = SqlReader::read_all(InlineResource::TABLE)
            .r#where(Wheres::and([
                Wheres::if_some(req.content_type.to_owned(), |e| {
                    Wheres::equal(InlineResource::CONTENT_TYPE, e)
                }),
                Wheres::if_some(req.id.to_owned(), |e| Wheres::equal(InlineResource::ID, e)),
                Wheres::if_some(req.name_like.to_owned(), |e| {
                    Wheres::ilike(InlineResource::NAME, e, chin_sql::ILikeType::Fuzzy)
                }),
                Wheres::if_some(
                    match req.with_del {
                        Some(true) => None,
                        _ => Some(()),
                    },
                    |_| Wheres::is_null(InlineResource::DELETE_TIME),
                ),
                Wheres::if_some(req.rid.to_owned(), |id| {
                    Wheres::equal(InlineResource::RID, id)
                }),
            ]))
            .raw("order by insert_time desc")
            .custom(LimitOffset::new(1));

        let res = self
            .conn()
            .await?
            .qry_list(query, |t| t.to_inline_resource())
            .await?;

        Ok(QueryInlineResourceRsp { res })
    }
}

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
        let query = SqlReader::read_all(KV::TABLE)
            .r#where(Wheres::and([Wheres::equal(KV::KEY, req.key.as_str())]));

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
        let del = SqlDeleter::new(KV::TABLE).r#where(Wheres::equal(KV::KEY, &req.key));

        self.conn().await?.exec(del).await?;

        Ok(KVDeleteRsp {})
    }

    async fn ensure_table_kv(&self) -> chin_tools::wrapper::anyhow::EResult {
        self.create_table(KV::schema(self.db_type())).await?;
        Ok(())
    }
}
