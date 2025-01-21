pub mod backup;
pub mod mappertype;
pub mod postgres;

use chin_tools::wrapper::anyhow::{AResult, EResult};
use postgres::{Postgres, PostgresConfig};
use serde::Deserialize;

use crate::model::{
    db::{
        namespace::{NamespaceRecord, NamespaceRelation},
        resource::Resource,
    },
    dto::{chnot::*, KReq},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MapperConfig {
    #[serde(rename = "postgres")]
    Postgres(PostgresConfig),
}

pub enum MapperType {
    Postgres(Postgres),
}

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


