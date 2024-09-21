use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::Local;

use crate::{
    mapper::NamespaceMapper,
    model::db::namespace::{NamespaceRecord, NamespaceRelation},
};

use super::Postgres;

impl NamespaceMapper for Postgres {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>> {
        let stmt = self.client().await?;
        let rows = stmt
            .query(
                "select * from namespace_record where delete_time is not null",
                &[],
            )
            .await?;
        let nrs = rows
            .iter()
            .map(|e| {
                Ok(NamespaceRecord {
                    id: e.try_get("id")?,
                    name: e.try_get("name")?,
                    delete_time: e.try_get("delete_time")?,
                    update_time: e.try_get("update_time")?,
                    insert_time: e.try_get("insert_time")?,
                })
            })
            .filter_map(|e: AResult<NamespaceRecord>| e.ok())
            .collect();

        Ok(nrs)
    }

    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>> {
        let stmt = self.client().await?;
        let rows = stmt
            .query(
                "select * from namespace_relation where delete_time is not null",
                &[],
            )
            .await?;
        let nrs = rows
            .iter()
            .map(|e| {
                Ok(NamespaceRelation {
                    id: e.try_get("id")?,
                    delete_time: e.try_get("delete_time")?,
                    update_time: e.try_get("update_time")?,
                    insert_time: e.try_get("insert_time")?,
                    sub_id: e.try_get("sub_id")?,
                    parent_id: e.try_get("parent_id")?,
                })
            })
            .filter_map(|e: AResult<NamespaceRelation>| e.ok())
            .collect();

        Ok(nrs)
    }

    async fn ensure_table_namespace_record(&self) -> EResult {
        self.create_table(
            "create table IF NOT EXISTS namespace_record (
    id VARCHAR(40) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    delete_time TIMESTAMPTZ,
    update_time TIMESTAMPTZ,
    insert_time TIMESTAMPTZ NOT NULL
)",
        )
        .await?;

        let fast_create = |name: &str| NamespaceRecord {
            id: name.to_owned(),
            name: name.to_owned(),
            delete_time: None,
            update_time: None,
            insert_time: Local::now().into(),
        };

        for v in [
            fast_create("private"),
            fast_create("public"),
            fast_create("work"),
        ] {
            self.client().await?.execute("insert into namespace_record(id, name, insert_time) values($1,$2,$3) on conflict do nothing", &[
                &v.id, &v.name, &v.insert_time
            ]).await?;
        }

        Ok(())
    }

    async fn ensure_table_namespace_relation(&self) -> EResult {
        self.create_table(
            "create table IF NOT EXISTS namespace_relation (
    id VARCHAR(40) PRIMARY KEY,
    sub_id varchar(40),
    parent_id varchar(40),
    delete_time TIMESTAMPTZ,
    update_time TIMESTAMPTZ,
    insert_time TIMESTAMPTZ NOT NULL
)",
        )
        .await
    }
}
