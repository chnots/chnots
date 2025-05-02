use chin_tools::wrapper::anyhow::{AResult, EResult};

use super::{DeserializeMapper, KDb, KDbBehaiver, KDbConnBehaiver};
use crate::{
    mapper::ResourceMapper,
    model::{
        db::resource::{InlineResource, Resource},
        dto::{InsertInlineResourceRsp, KReq, QueryInlineResourceRsp},
    },
};

use chin_sql::{SqlInserter, SqlReader, Wheres};

impl ResourceMapper for KDb {
    async fn ensure_table_resource(&self) -> EResult {
        self.create_table(Resource::schema(self.db_type()))
            .await
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
                SqlReader::read_all(Resource::TABLE)
                    .r#where(Wheres::equal(Resource::ID, id)),
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
                SqlInserter::new(InlineResource::TABLE)
                    .fields(InlineResource::ID, &req.res.id)
                    .fields(InlineResource::NAME, &req.res.name)
                    .fields(InlineResource::CONTENT, &req.res.name)
                    .fields(InlineResource::INSERT_TIME, &req.res.insert_time),
            )
            .await?;

        Ok(InsertInlineResourceRsp {})
    }

    async fn query_inline_resource(
        &self,
        req: KReq<crate::model::dto::QueryInlineResourceReq>,
    ) -> anyhow::Result<crate::model::dto::QueryInlineResourceRsp> {
        let query = SqlReader::read_all(InlineResource::TABLE)
            .r#where(Wheres::and([
                Wheres::is_null(InlineResource::DELETE_TIME),
                Wheres::if_some(req.content_type.to_owned(), |e| {
                    Wheres::equal(InlineResource::CONTENT_TYPE, e)
                }),
                Wheres::if_some(req.id.to_owned(), |e| Wheres::equal(InlineResource::ID, e)),
                Wheres::if_some(req.name_like.to_owned(), |e| {
                    Wheres::ilike(InlineResource::NAME, e, chin_sql::ILikeType::Fuzzy)
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
