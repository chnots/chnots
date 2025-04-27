use chin_tools::wrapper::anyhow::{AResult, EResult};

use super::{DeserializeMapper, KDb, KDbBehaiver, KDbConnBehaiver};
use crate::{
    mapper::ResourceMapper,
    model::{
        db::resource::{InlineResource, Resource},
        dto::{InsertInlineResourceRsp, KReq, QueryInlineResourceRsp},
    },
};

use chin_sql::{SqlInserter, SqlSegBuilder, Wheres};

impl ResourceMapper for KDb {
    async fn ensure_table_resource(&self) -> EResult {
        self.create_table(Resource::table_creation_sql(self.db_type()))
            .await
    }

    async fn ensure_table_inline_resource(&self) -> EResult {
        self.create_table(InlineResource::table_creation_sql(self.db_type()))
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
            SqlInserter::new(Resource::table_name())
                .fields(Resource::field_id(), id.to_owned())
                .fields(Resource::field_ori_filename(), ori_filename.to_owned())
                .fields(Resource::field_namespace(), namespace.to_owned())
                .fields(Resource::field_content_type(), content_type.to_owned())
                .fields(Resource::field_insert_time(), insert_time.to_owned()),
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
                SqlSegBuilder::new()
                    .raw("select * from resources")
                    .r#where(Wheres::equal(Resource::field_id(), id)),
                |e| e.to_resource(),
                false,
            )
            .await?;
        Ok(res)
    }

    async fn insert_inline_resource(
        &self,
        req: &KReq<crate::model::dto::InsertInlineResourceReq>,
    ) -> anyhow::Result<InsertInlineResourceRsp> {
        self.conn()
            .await?
            .exec(
                SqlInserter::new(InlineResource::table_name())
                    .fields(InlineResource::field_id(), &req.res.id)
                    .fields(InlineResource::field_name(), &req.res.name)
                    .fields(InlineResource::field_content(), &req.res.name)
                    .fields(InlineResource::field_insert_time(), &req.res.insert_time),
            )
            .await?;

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
                Wheres::if_some(req.name_like.to_owned(), |e| {
                    Wheres::ilike("name", e, self.db_type())
                }),
            ]))
            .raw("order by insert_time desc");

        let res = self
            .conn()
            .await?
            .qry_list(query, |t| t.to_inline_resource())
            .await?;

        Ok(QueryInlineResourceRsp { res })
    }
}
