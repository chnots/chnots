use chin_sql::{SqlInserter, SqlReader, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;

use super::{DeserializeMapper, KDb, KDbBehaiver, KDbConnBehaiver};
use crate::{
    mapper::WorkspaceMapper,
    model::db::workspace::{WorkspaceRecord, WorkspaceRelation},
};

impl WorkspaceMapper for KDb {
    async fn read_all_workspaces(&self) -> AResult<Vec<WorkspaceRecord>> {
        let stmt = self.conn().await?;
        stmt.qry_list(
            SqlReader::read_all(WorkspaceRecord::TABLE)
                .r#where(Wheres::is_not_null(WorkspaceRecord::DELETE_TIME)),
            |e| e.to_workspace_record(),
        )
        .await
    }

    async fn read_all_workspace_relations(&self) -> AResult<Vec<WorkspaceRelation>> {
        self.conn()
            .await?
            .qry_list(
                SqlReader::read_all(WorkspaceRelation::TABLE)
                .r#where(Wheres::is_not_null(WorkspaceRelation::DELETE_TIME)),                |e| e.to_workspace_relation(),
            )
            .await
    }

    async fn ensure_table_workspace_record(&self) -> EResult {
        self.create_table(WorkspaceRecord::schema(self.db_type()))
            .await?;

        let fast_create = |name: &str| WorkspaceRecord {
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
            let inserter = SqlInserter::new(WorkspaceRecord::TABLE)
                .fields(WorkspaceRecord::ID, &v.id)
                .fields(WorkspaceRecord::NAME, &v.name)
                .fields(WorkspaceRecord::INSERT_TIME, &v.insert_time)
                .on_conflict(chin_sql::OnConflict::Ignore);

            self.conn().await?.exec(inserter).await?;
        }

        Ok(())
    }

    async fn ensure_table_workspace_relation(&self) -> EResult {
        self.create_table(WorkspaceRelation::schema(self.db_type()))
            .await
    }
}
