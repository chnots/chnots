use chin_tools::{AResult, EResult};

use crate::{
    expand_mt_branch,
    mapper::db::{KDbRow, KDbRowBehavier},
    MapperType,
};

use super::*;

pub(crate) trait KSpaceDeserializeMapper {
    fn to_kspace_record(self) -> AResult<KSpaceRecord>;
    fn to_kspace_relation(self) -> AResult<KSpaceRelation>;
}

pub(crate) trait KSpaceMapper {
    async fn read_all_kspaces(&self) -> AResult<Vec<KSpaceRecord>>;
    async fn read_all_kspace_relations(&self) -> AResult<Vec<KSpaceRelation>>;

    async fn ensure_table_kspace_record(&self) -> EResult;
    async fn ensure_table_kspace_relation(&self) -> EResult;
    async fn ensure_table_kspace(&self) -> EResult {
        self.ensure_table_kspace_record().await?;
        self.ensure_table_kspace_relation().await?;

        Ok(())
    }
}

impl KSpaceDeserializeMapper for KDbRow {
    fn to_kspace_record(self) -> AResult<KSpaceRecord> {
        let obj = KSpaceRecord {
            id: self.try_get(KSpaceRecord::ID)?,
            insert_time: self.try_get(KSpaceRecord::INSERT_TIME)?,
            name: self.try_get(KSpaceRecord::NAME)?,
            delete_time: self.try_get(KSpaceRecord::DELETE_TIME)?,
            update_time: self.try_get(KSpaceRecord::UPDATE_TIME)?,
        };
        Ok(obj)
    }

    fn to_kspace_relation(self) -> AResult<KSpaceRelation> {
        let obj = KSpaceRelation {
            id: self.try_get(KSpaceRelation::ID)?,
            insert_time: self.try_get(KSpaceRelation::INSERT_TIME)?,
            delete_time: self.try_get(KSpaceRelation::DELETE_TIME)?,
            update_time: self.try_get(KSpaceRelation::UPDATE_TIME)?,
            sub_id: self.try_get(KSpaceRelation::SUB_ID)?,
            parent_id: self.try_get(KSpaceRelation::PARENT_ID)?,
        };
        Ok(obj)
    }
}

impl KSpaceMapper for MapperType {
    async fn read_all_kspaces(&self) -> AResult<Vec<KSpaceRecord>> {
        expand_mt_branch!(self.read_all_kspaces())
    }

    async fn read_all_kspace_relations(&self) -> AResult<Vec<KSpaceRelation>> {
        expand_mt_branch!(self.read_all_kspace_relations())
    }

    async fn ensure_table_kspace_record(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kspace_record())
    }

    async fn ensure_table_kspace_relation(&self) -> EResult {
        expand_mt_branch!(self.ensure_table_kspace_relation())
    }
}
