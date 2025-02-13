pub mod backup;
pub mod chnot;
pub mod kv;
pub mod llmchat;
pub mod namespace;
pub mod resource;

use chin_tools::wrapper::anyhow::{AResult, EResult};
use deadpool_postgres::{Client, Pool, PoolError};
use postgres_types::{to_sql_checked, FromSql, ToSql};
use serde::Deserialize;
use tokio_postgres::Row;

use crate::{
    model::{
        db::{chnot::*, kv::KV, llmchat::*, namespace::*, resource::Resource},
        shared_str::SharedStr,
    },
    util::sql_builder::SqlValue,
};

use super::DeserializeMapper;

const NO_PARAMS: Vec<&(dyn ToSql + Sync)> = Vec::new();

#[derive(Debug, Deserialize, Clone)]
pub struct PostgresConfig {
    user: String,
    pass: String,
    dbname: String,
    host: String,
    port: u16,
}

impl Into<deadpool_postgres::Config> for PostgresConfig {
    fn into(self) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.user = Some(self.user);
        cfg.password = Some(self.pass);
        cfg.dbname = Some(self.dbname);
        cfg.host = Some(self.host);
        cfg.port = Some(self.port);
        cfg
    }
}

pub struct Postgres {
    pub pool: Pool,
}

impl Postgres {
    pub fn new(config: PostgresConfig) -> AResult<Postgres> {
        let pool = Into::<deadpool_postgres::Config>::into(config)
            .create_pool(None, tokio_postgres::NoTls)?;

        Ok(Postgres { pool })
    }

    async fn client(&self) -> Result<Client, PoolError> {
        self.pool.get().await
    }

    async fn create_table(&self, create_sql: &str) -> EResult {
        self.client()
            .await?
            .execute(create_sql, &[])
            .await
            .map(|_| ())
            .map_err(anyhow::Error::new)?;

        Ok(())
    }
}

impl<'a> Into<&'a (dyn ToSql + Sync + Send)> for &'a SqlValue<'a> {
    fn into(self) -> &'a (dyn ToSql + Sync + Send) {
        match self {
            SqlValue::I8(v) => v,
            SqlValue::I16(v) => v,
            SqlValue::I32(v) => v,
            SqlValue::I64(v) => v,
            SqlValue::Str(v) => v,
            SqlValue::Date(v) => v.as_ref(),
            SqlValue::Bool(v) => v,
            SqlValue::Opt(v) => match v {
                Some(v) => v.as_ref().into(),
                None => &None::<String>,
            },
            SqlValue::SharedStr(shared_str) => shared_str,
        }
    }
}

impl ToSql for SharedStr {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut tokio_util::bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for SharedStr {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <&str as tokio_postgres::types::FromSql>::from_sql(ty, raw)
            .and_then(|s| Ok(SharedStr::new(s)))
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        <&str as tokio_postgres::types::FromSql>::accepts(ty)
    }
}

#[macro_export]
macro_rules! to_sql {
    ($values:expr) => {
        $values
            .iter()
            .map(|e| {
                let v: &(dyn postgres_types::ToSql + Sync + Send) = e.into();
                v as &(dyn postgres_types::ToSql + Sync)
            })
            .collect::<Vec<&(dyn postgres_types::ToSql + Sync)>>()
            .as_slice()
    };
}

impl DeserializeMapper for Postgres {
    type RowType = Row;

    fn to_chnot_meta(row: Self::RowType) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: row.try_get("id")?,
            namespace: row.try_get("namespace")?,
            kind: row.try_get("kind")?,
            pin_time: row.try_get("pin_time")?,
            delete_time: row.try_get("delete_time")?,
            update_time: row.try_get("update_time")?,
            insert_time: row.try_get("insert_time")?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(row: Self::RowType) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: row.try_get("id")?,
            meta_id: row.try_get("meta_id")?,
            content: row.try_get("content")?,
            omit_time: row.try_get("omit_time")?,
            insert_time: row.try_get("insert_time")?,
        };
        Ok(chnot)
    }

    fn to_llmchat_bot(row: Self::RowType) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            delete_time: row.try_get("delete_time")?,
            name: row.try_get("name")?,
            body: row.try_get("body")?,
            update_time: row.try_get("update_time")?,
            svg_logo: row.try_get("svg_logo")?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(row: Self::RowType) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            delete_time: row.try_get("delete_time")?,
            update_time: row.try_get("update_time")?,
            name: row.try_get("name")?,
            prompt: row.try_get("prompt")?,
            svg_logo: row.try_get("svg_logo")?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(row: Self::RowType) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            bot_id: row.try_get("bot_id")?,
            template_id: row.try_get("template_id")?,
            title: row.try_get("title")?,
            namespace: row.try_get("namespace")?,
            delete_time: row.try_get("delete_time")?,
            update_time: row.try_get("update_time")?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(row: Self::RowType) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            session_id: row.try_get("session_id")?,
            pre_record_id: row.try_get("pre_record_id")?,
            content: row.try_get("content")?,
            role: row.try_get("role")?,
            role_id: row.try_get("role_id")?,
        };
        Ok(obj)
    }

    fn to_namespace_record(row: Self::RowType) -> AResult<NamespaceRecord> {
        let obj = NamespaceRecord {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            name: row.try_get("name")?,
            delete_time: row.try_get("delete_time")?,
            update_time: row.try_get("update_time")?,
        };
        Ok(obj)
    }

    fn to_namespace_relation(row: Self::RowType) -> AResult<NamespaceRelation> {
        let obj = NamespaceRelation {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            delete_time: row.try_get("delete_time")?,
            update_time: row.try_get("update_time")?,
            sub_id: row.try_get("sub_id")?,
            parent_id: row.try_get("parent_id")?,
        };
        Ok(obj)
    }

    fn to_resource(row: Self::RowType) -> AResult<Resource> {
        let obj = Resource {
            id: row.try_get("id")?,
            insert_time: row.try_get("insert_time")?,
            delete_time: row.try_get("delete_time")?,
            namespace: row.try_get("namespace")?,
            ori_filename: row.try_get("ori_filename")?,
            content_type: row.try_get("content_type")?,
        };
        Ok(obj)
    }

    fn to_kv(row: Self::RowType) -> AResult<KV> {
        let obj = KV {
            insert_time: row.try_get("insert_time")?,
            key: row.try_get("key")?,
            value: row.try_get("value")?,
            update_time: row.try_get("update_time")?,
        };
        Ok(obj)
    }
}
