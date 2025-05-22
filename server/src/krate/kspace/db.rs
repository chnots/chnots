use chin_sql::{SqlInserter, SqlReader, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;

use crate::mapper::db::{KDb, KDbBehaiver, KDbConnBehaiver};

use super::{mapper::*, *};

impl KSpaceMapper for KDb {
    async fn read_all_kspaces(&self) -> AResult<Vec<KSpaceRecord>> {
        let stmt = self.conn().await?;
        stmt.qry_list(
            SqlReader::read_all(KSpaceRecord::TABLE)
                .r#where(Wheres::is_not_null(KSpaceRecord::DELETE_TIME)),
            |e| e.to_kspace_record(),
        )
        .await
    }

    async fn read_all_kspace_relations(&self) -> AResult<Vec<KSpaceRelation>> {
        self.conn()
            .await?
            .qry_list(
                SqlReader::read_all(KSpaceRelation::TABLE)
                    .r#where(Wheres::is_not_null(KSpaceRelation::DELETE_TIME)),
                |e| e.to_kspace_relation(),
            )
            .await
    }

    async fn ensure_table_kspace_record(&self) -> EResult {
        self.create_table(KSpaceRecord::schema(self.db_type()))
            .await?;

        let fast_create = |name: &str| KSpaceRecord {
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
            let inserter = SqlInserter::new(KSpaceRecord::TABLE)
                .fields(KSpaceRecord::ID, &v.id)
                .fields(KSpaceRecord::NAME, &v.name)
                .fields(KSpaceRecord::INSERT_TIME, v.insert_time)
                .on_conflict(chin_sql::OnConflict::Ignore);

            self.conn().await?.exec(inserter).await?;
        }

        Ok(())
    }

    async fn ensure_table_kspace_relation(&self) -> EResult {
        self.create_table(KSpaceRelation::schema(self.db_type()))
            .await
    }
}
