use chin_sql::SqlInserter;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::Local;

use super::{DeserializeMapper, KDb, KDbBehaiver, KDbConnBehaiver};
use crate::{
    mapper::NamespaceMapper,
    model::db::namespace::{NamespaceRecord, NamespaceRelation},
};

impl NamespaceMapper for KDb {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>> {
        let stmt = self.conn().await?;
        stmt.qry_list(
            "select * from namespace_record where delete_time is not null",
            |e| e.to_namespace_record(),
        )
        .await
    }

    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>> {
        self.conn()
            .await?
            .qry_list(
                "select * from namespace_relation where delete_time is not null",
                |e| e.to_namespace_relation(),
            )
            .await
    }

    async fn ensure_table_namespace_record(&self) -> EResult {
        self.create_table(NamespaceRecord::table_creation_sql(self.db_type()))
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
            let inserter = SqlInserter::new(NamespaceRecord::table_name())
                .fields(NamespaceRecord::field_id(), &v.id)
                .fields(NamespaceRecord::field_name(), &v.name)
                .fields(NamespaceRecord::field_insert_time(), &v.insert_time)
                .on_conflict(chin_sql::OnConflict::Ignore);

            self.conn().await?.exec(inserter).await?;
        }

        Ok(())
    }

    async fn ensure_table_namespace_relation(&self) -> EResult {
        self.create_table(NamespaceRelation::table_creation_sql(self.db_type()))
            .await
    }
}
