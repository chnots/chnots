use chin_tools::wrapper::anyhow::{AResult, EResult};

use crate::{mapper::ResourceMapper, model::db::resource::Resource};

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

        Ok(Resource {
            id: row.try_get("id")?,
            namespace: row.try_get("namespace")?,
            ori_filename: row.try_get("ori_filename")?,
            content_type: row.try_get("content_type")?,
            insert_time: row.try_get("insert_time")?,
            delete_time: row.try_get("delete_time")?,
        })
    }
}
