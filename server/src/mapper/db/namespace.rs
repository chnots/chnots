use chin_sql::{SqlInserter, SqlReader, Wheres};
use chin_tools::{AResult, EResult};
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
            SqlReader::read_all(NamespaceRecord::TABLE)
                .r#where(Wheres::is_not_null(NamespaceRecord::DELETE_TIME)),
            |e| e.to_namespace_record(),
        )
        .await
    }

    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>> {
        self.conn()
            .await?
            .qry_list(
                SqlReader::read_all(NamespaceRelation::TABLE)
                .r#where(Wheres::is_not_null(NamespaceRelation::DELETE_TIME)),                |e| e.to_namespace_relation(),
            )
            .await
    }

    async fn ensure_table_namespace_record(&self) -> EResult {
        self.create_table(NamespaceRecord::schema(self.db_type()))
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
            let inserter = SqlInserter::new(NamespaceRecord::TABLE)
                .fields(NamespaceRecord::ID, &v.id)
                .fields(NamespaceRecord::NAME, &v.name)
                .fields(NamespaceRecord::INSERT_TIME, &v.insert_time)
                .on_conflict(chin_sql::OnConflict::Ignore);

            self.conn().await?.exec(inserter).await?;
        }

        Ok(())
    }

    async fn ensure_table_namespace_relation(&self) -> EResult {
        self.create_table(NamespaceRelation::schema(self.db_type()))
            .await
    }
}
