use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    mapper::db::{KDbRow, KDbRowBehavier},
    MapperType,
};

use super::*;

pub(crate) trait KSpaceDeserializeMapper {
    fn to_workspace_record(self) -> AResult<WorkspaceRecord>;
    fn to_workspace_relation(self) -> AResult<WorkspaceRelation>;
}

pub(crate) trait WorkspaceMapper {
    async fn read_all_workspaces(&self) -> AResult<Vec<WorkspaceRecord>>;
    async fn read_all_workspace_relations(&self) -> AResult<Vec<WorkspaceRelation>>;

    async fn ensure_table_workspace_record(&self) -> EResult;
    async fn ensure_table_workspace_relation(&self) -> EResult;
    async fn ensure_table_workspace(&self) -> EResult {
        self.ensure_table_workspace_record().await?;
        self.ensure_table_workspace_relation().await?;

        Ok(())
    }
}

impl KSpaceDeserializeMapper for KDbRow<'_> {
    fn to_workspace_record(self) -> AResult<WorkspaceRecord> {
        let obj = WorkspaceRecord {
            id: self.try_get(WorkspaceRecord::ID)?,
            insert_time: self.try_get(WorkspaceRecord::INSERT_TIME)?,
            name: self.try_get(WorkspaceRecord::NAME)?,
            delete_time: self.try_get(WorkspaceRecord::DELETE_TIME)?,
            update_time: self.try_get(WorkspaceRecord::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_workspace_relation(self) -> AResult<WorkspaceRelation> {
        let obj = WorkspaceRelation {
            id: self.try_get(WorkspaceRelation::ID)?,
            insert_time: self.try_get(WorkspaceRelation::INSERT_TIME)?,
            delete_time: self.try_get(WorkspaceRelation::DELETE_TIME)?,
            update_time: self.try_get(WorkspaceRelation::UPDATE_TIME)?,
            sub_id: self.try_get(WorkspaceRelation::SUB_ID)?,
            parent_id: self.try_get(WorkspaceRelation::PARENT_ID)?,
        };
        Ok(obj)
    }
}

impl WorkspaceMapper for MapperType {
    async fn read_all_workspaces(&self) -> AResult<Vec<WorkspaceRecord>> {
        expand_mt_branch!(self.read_all_workspaces())
    }

    async fn read_all_workspace_relations(&self) -> AResult<Vec<WorkspaceRelation>> {
        expand_mt_branch!(self.read_all_workspace_relations())
    }

    async fn ensure_table_workspace_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_workspace_record())
    }

    async fn ensure_table_workspace_relation(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_workspace_relation())
    }
}
