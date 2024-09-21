pub mod backup;
pub mod postgres;

use std::future::Future;

use anyhow::Ok;
use backup::{BackupTrait, DumpWrapper};
use chin_tools::wrapper::anyhow::{AResult, EResult};
use postgres::{resource, Postgres, PostgresConfig};
use serde::Deserialize;

use crate::model::{
    db::{
        chnot::ChnotRecord,
        namespace::{NamespaceRecord, NamespaceRelation},
        resource::Resource,
    },
    dto::*,
};

pub trait ChnotMapper {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp>;
    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp>;
    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>>;
    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp>;

    async fn ensure_table_chnot_record(&self) -> EResult;
    async fn ensure_table_chnot_metadata(&self) -> EResult;
}

pub trait ResourceMapper {
    async fn insert_resource(&self, resource: &Resource) -> anyhow::Result<Resource>;
    async fn query_resource_by_id(&self, id: &str) -> anyhow::Result<Resource>;

    async fn ensure_table_resource(&self) -> EResult;
}

pub trait NamespaceMapper {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>>;
    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>>;

    async fn ensure_table_namespace_record(&self) -> EResult;
    async fn ensure_table_namespace_relation(&self) -> EResult;
}

pub enum MapperType {
    Postgres(Postgres),
}

impl MapperType {
    pub async fn ensure_tables(&self) -> EResult {
        self.ensure_table_chnot_record().await?;
        self.ensure_table_namespace_record().await?;
        self.ensure_table_namespace_relation().await?;
        self.ensure_table_chnot_metadata().await?;
        self.ensure_table_resource().await?;

        Ok(())
    }
}

impl ChnotMapper for MapperType {
    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        match self {
            MapperType::Postgres(db) => db.chnot_overwrite(req).await,
        }
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        match self {
            MapperType::Postgres(db) => db.chnot_delete(req).await,
        }
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        match self {
            MapperType::Postgres(db) => db.chnot_query(req).await,
        }
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        match self {
            MapperType::Postgres(db) => db.chnot_update(req).await,
        }
    }

    async fn ensure_table_chnot_record(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_chnot_record().await,
        }
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_chnot_metadata().await,
        }
    }
}

impl ResourceMapper for MapperType {
    async fn insert_resource(&self, resource: &Resource) -> anyhow::Result<Resource> {
        match self {
            MapperType::Postgres(db) => db.insert_resource(resource).await,
        }
    }

    async fn query_resource_by_id(&self, id: &str) -> anyhow::Result<Resource> {
        match self {
            MapperType::Postgres(db) => db.query_resource_by_id(id).await,
        }
    }

    async fn ensure_table_resource(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_resource().await,
        }
    }
}

impl NamespaceMapper for MapperType {
    async fn read_all_namespaces(&self) -> AResult<Vec<NamespaceRecord>> {
        match self {
            MapperType::Postgres(db) => db.read_all_namespaces().await,
        }
    }

    async fn read_all_namespace_relations(&self) -> AResult<Vec<NamespaceRelation>> {
        match self {
            MapperType::Postgres(db) => db.read_all_namespace_relations().await,
        }
    }

    async fn ensure_table_namespace_record(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_namespace_record().await,
        }
    }

    async fn ensure_table_namespace_relation(&self) -> EResult {
        match self {
            MapperType::Postgres(db) => db.ensure_table_namespace_relation().await,
        }
    }
}

impl BackupTrait for MapperType {
    async fn dump_chnots<F, R1>(&self, _: F) -> EResult
    where
        F: Fn(DumpWrapper<ChnotRecord>) -> R1,
        R1: Future<Output = EResult>,
    {
        match self {
            MapperType::Postgres(_) => {
                tracing::error!("not implement dump logic");
                Ok(())
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
}

impl Into<AResult<MapperType>> for MapperConfig {
    fn into(self) -> AResult<MapperType> {
        match self {
            MapperConfig::Postgres(config) => {
                let pg = Postgres::new(config)?;
                Ok(MapperType::Postgres(pg))
            }
        }
    }
}
