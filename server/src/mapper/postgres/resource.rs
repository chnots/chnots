use anyhow::Context;
use chin_tools::wrapper::anyhow::{AResult, EResult};

use super::DeserializeMapper;
use crate::{
    mapper::ResourceMapper,
    model::{
        db::resource::{InlineResource, Resource},
        dto::{InsertInlineResourceRsp, KReq, QueryInlineResourceRsp},
    },
    to_sql,
    util::sql_builder::{PlaceHolderType, SqlSegBuilder, Wheres},
};

use super::Postgres;

impl ResourceMapper for Postgres {
    async fn ensure_table_resource(&self) -> EResult {
        self.create_table(
            "create table IF NOT EXISTS resources (
    id VARCHAR(40) PRIMARY KEY,

    namespace VARCHAR(100) NOT NULL,
    ori_filename VARCHAR(300) NOT NULL,

    content_type VARCHAR(100) NOT NULL,

    delete_time TIMESTAMPTZ,
    insert_time TIMESTAMPTZ NOT NULL
)",
        )
        .await
    }

    async fn ensure_table_inline_resource(&self) -> EResult {
        self.create_table(
            "create table IF NOT EXISTS inline_resource (
    id VARCHAR(40) PRIMARY KEY,

    name VARCHAR(300) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    
    delete_time TIMESTAMPTZ,
    insert_time TIMESTAMPTZ NOT NULL
)",
        )
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

        let stmt = self.pool.get().await?;

        let insert_time = chrono::Utc::now().to_owned();

        stmt.execute(
            "insert into resources(id, namespace, ori_filename, content_type, insert_time) values ($1,$2,$3,$4, $5)",
            &[&id, &namespace, &ori_filename, &content_type, &insert_time]
        ).await
            .map_err(|e| anyhow::Error::new(e))
            .map(|_| Resource {
                id: id.to_owned(),
                namespace: namespace.to_owned(),
                ori_filename: ori_filename.to_string(),
                content_type: content_type.to_owned(),
                insert_time,
                delete_time: None,
            })
    }

    async fn query_resource_by_id(&self, id: &str) -> AResult<Resource> {
        let stmt = self.pool.get().await?;
        let row = stmt
            .query_one("select * from resources where id = $1", &[&id])
            .await?;

        Self::to_resource(row)
    }

    async fn insert_inline_resource(
        &self,
        req: &KReq<crate::model::dto::InsertInlineResourceReq>,
    ) -> anyhow::Result<InsertInlineResourceRsp> {
        self.client().await?
        .execute(
            "insert into inline_resource(id, name, content, content_type, insert_time) values ($1,$2,$3,$4,$5)",
            &[
                &req.res.id,
                &req.res.name,
                &req.res.content,
                &req.res.content_type,
                &req.res.insert_time
            ]
        ).await?;

        Ok(InsertInlineResourceRsp {})
    }

    async fn query_inline_resource(
        &self,
        req: KReq<crate::model::dto::QueryInlineResourceReq>,
    ) -> anyhow::Result<crate::model::dto::QueryInlineResourceRsp> {
        let query = SqlSegBuilder::new()
            .raw("select * from inline_resource")
            .r#where(Wheres::and([
                Wheres::is_null("delete_time"),
                Wheres::if_some(req.content_type.to_owned(), |e| {
                    Wheres::equal("content_type", e)
                }),
                Wheres::if_some(req.id.to_owned(), |e| Wheres::equal("id", e)),
                Wheres::if_some(req.name_like.to_owned(), |e| Wheres::ilike("name", e)),
            ]))
            .raw("order by insert_time desc")
            .build(&mut PlaceHolderType::dollar_number())
            .context("Unable to build args")?;

        let res: AResult<Vec<InlineResource>> = self
            .client()
            .await?
            .query(query.seg.as_str(), to_sql!(query.values))
            .await?
            .iter()
            .map(|t| {
                let r = InlineResource {
                    id: t.try_get("id")?,
                    name: t.try_get("name")?,
                    delete_time: t.try_get("delete_time")?,
                    insert_time: t.try_get("insert_time")?,
                    content: t.try_get("content")?,
                    content_type: t.try_get("content_type")?,
                };
                Ok(r)
            })
            .collect();

        Ok(QueryInlineResourceRsp { res: res? })
    }
}
