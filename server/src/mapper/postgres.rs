use std::{borrow::Cow, future::Future, str::FromStr, vec};

use crate::{
    model::v1::db::{
        chnot::{Chnot, ChnotHierarchy, ChnotType},
        resource::Resource,
    },
    to_sql,
    util::sql_param_builder::{SimpleUpdater, SqlQuery, SqlValue, ValueType, Wheres},
};
use anyhow::Context;
use chin_tools::{
    utils::idutils,
    wrapper::anyhow::{AResult, EResult},
};
use chrono::Local;
use deadpool_postgres::{Client, Pool, PoolError};
use futures::{pin_mut, TryStreamExt};
use postgres_types::{to_sql_checked, FromSql, ToSql};
use serde::Deserialize;
use tokio_postgres::Row;
use tracing::info;

use crate::model::v1::dto::*;

use super::{
    backup::{BackupTrait, DumpWrapper},
    ResourceMapper,
};
use super::{ChnotMapper, TableFounder};

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

fn map_row_to_chnot(row: &Row) -> AResult<Chnot> {
    let chnot = Chnot {
        id: row.try_get("id")?,
        perm_id: row.try_get("perm_id")?,
        content: {
            let s: String = row.try_get("content")?;
            Cow::Owned(s)
        },
        r#type: row.try_get("type")?,
        domain: row.try_get("domain")?,
        delete_time: row.try_get("delete_time")?,
        insert_time: row.try_get("insert_time")?,
        pinned: row.try_get("pinned")?,
        archive_time: row.try_get("archive_time")?,
        init_time: row.try_get("init_time")?,
        rind_id: row.try_get("ring_id")?,
    };

    Ok(chnot)
}

fn map_row_to_chnot_hierarchy(row: &Row) -> AResult<ChnotHierarchy> {
    let chnot = ChnotHierarchy {
        id: row.try_get("id")?,

        insert_time: row.try_get("insert_time")?,
        parent_id: row.try_get("partent_id")?,
        prev_id: row.try_get("prev_id")?,
        chnot_id: row.try_get("chnot_id")?,
    };

    Ok(chnot)
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

    async fn check_if_table_not_exists(&self, table_name: &str) -> AResult<bool> {
        let client = self.client().await?;
        client
            .query(
                "SELECT 1 FROM pg_tables WHERE  schemaname = 'public' AND tablename = $1",
                &[&table_name.to_string()],
            )
            .await
            .map(|e| e.is_empty())
            .map_err(anyhow::Error::new)
    }

    async fn create_table(&self, table_name: &str, create_sql: &str) -> EResult {
        info!("begin to create table `{}'", table_name);
        if self.check_if_table_not_exists(table_name).await? {
            info!("table `{}' is not existed.", table_name);
            let client = self.client().await?;
            client
                .execute(create_sql, &[])
                .await
                .map(|_| ())
                .map_err(anyhow::Error::new)?;
        } else {
            info!("table {} is already created.", table_name);
        }
        Ok(())
    }
}

impl TableFounder for Postgres {
    async fn _ensure_table_chnots(&self) -> EResult {
        self.create_table(
            "chnots",
            "create table chnots (
    id VARCHAR(40) NOT NULL,
    perm_id VARCHAR(40) NOT NULL,
    ring_id VARCHAR(40) NOT NULL,

    content TEXT NOT NULL,
    type VARCHAR(255) NOT NULL,
    domain TEXT NOT NULL,

    pinned bool not null default false,

    delete_time timestamptz DEFAULT NULL,
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    init_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    primary key (id)
)",
        )
        .await
    }

    async fn _ensure_table_toent_defi(&self) -> EResult {
        Ok(())
    }

    async fn _ensure_table_toent_inst(&self) -> EResult {
        Ok(())
    }

    async fn _ensure_table_llm_chat(&self) -> EResult {
        self.create_table(
            "llm_chat_history_v1",
            "CREATE TABLE llm_chat_history_v1 (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            serie_id TEXT NOT NULL,
            template_id TEXT NOT NULL,
            llm_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            insert_time TIMESTAMPTZ NOT NULL
        )",
        )
        .await?;

        self.create_table(
            "llm_chat_config",
            "CREATE TABLE llm_chat_config (
                id TEXT PRIMARY KEY,
                config TEXT NOT NULL,
                delete_time TIMESTAMPTZ,
                insert_time TIMESTAMPTZ NOT NULL,
                update_time TIMESTAMPTZ NOT NULL
                )",
        )
        .await?;

        self.create_table(
            "llm_chat_template",
            "CREATE TABLE llm_chat_template (
                id TEXT PRIMARY KEY,
                template TEXT NOT NULL,
                delete_time TIMESTAMPTZ,
                insert_time TIMESTAMPTZ NOT NULL,
                update_time TIMESTAMPTZ NOT NULL
                )",
        )
        .await?;

        Ok(())
    }

    async fn _ensure_table_resources(&self) -> EResult {
        self.create_table(
            "resources",
            "create table resources (
    id VARCHAR(40) PRIMARY KEY,

    domain VARCHAR(100) NOT NULL,
    ori_filename VARCHAR(300) NOT NULL,

    content_type VARCHAR(100) NOT NULL,

    delete_time TIMESTAMPTZ,
    insert_time TIMESTAMPTZ NOT NULL
)",
        )
        .await
    }

    async fn _ensure_table_chnot_hierarchies(&self) -> EResult {
        self.create_table(
            "chnot_hierarchies",
            "create table chnot_hierarchies (
    id VARCHAR(40) NOT NULL,

    chnot_id VARCHAR(40) NOT NULL,
    prev_id VARCHAR(40) comment 'prev chnot id',
    parent_id VARCHAR(40) comment 'parent chnot id',

    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP
)",
        )
        .await
    }
}

impl ChnotMapper for Postgres {
    async fn chnot_overwrite(&self, req: KReq<ChnotInsertionReq>) -> AResult<ChnotInsertionRsp> {
        let chnot = &req.body.chnot;

        let mut client = self.client().await?;

        let transaction = client.build_transaction().start().await?;
        let new_id = idutils::generate_uuid();

        let old_id = transaction
            .query_one(
                "select id form chnots where perm_id = $1 and delete_time is null",
                &[&req.chnot.perm_id],
            )
            .await
            .and_then(|row| {
                let id: String = row.try_get("id")?;
                Ok(id)
            })?;

        transaction
            .execute(
                "update chnots set delete_time = CURRENT_TIMESTAMP where id = $1",
                &[&old_id],
            )
            .await?;

        transaction.execute(
            "insert into chnots(id, perm_id, ring_id, pinned, content, type, domain, insert_time) values($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &chnot.id,
                &chnot.perm_id,
                &chnot.pinned,
                &chnot.content,
                &chnot.r#type,
                &chnot.domain,
                &chnot.insert_time,
            ]
        ).await?;

        let hierarchies: Vec<ChnotHierarchy> = transaction
            .query(
                "select * from chnot_hierarchies where id = $1 or parent_id = $1 or prev_id = $1",
                &[&old_id],
        )
            .await?
            .iter()
            .filter_map(|row| map_row_to_chnot_hierarchy(row).ok())
            .filter_map(|mut ch| {
                if ch.id == old_id {
                    ch.id = new_id.clone();
                } else if ch.prev_id.as_ref().map_or(false, |v| v == &old_id) {
                    ch.prev_id.replace(new_id.clone());
                } else if ch.parent_id.as_ref().map_or(false, |v| v == &old_id) {
                    ch.parent_id.replace(new_id.clone());
                };

                ch.insert_time = chnot.insert_time.clone();

                Some(ch)
            })
            .collect();

        for h in hierarchies {
            transaction.execute(
                "insert into chnot_hierarchies (id, chnot_id, parent_id, prev_id, insert_time) values($1, $2, $3, $4, $5)",
                &[
                    &h.id,
                    &h.chnot_id,
                    &h.parent_id,
                    &h.prev_id,
                    &h.insert_time
                ]
            ).await?;
        }

        Ok(ChnotInsertionRsp {})
    }

    async fn chnot_delete(
        &self,
        req: KReq<super::ChnotDeletionReq>,
    ) -> AResult<super::ChnotDeletionRsp> {
        let client = self.client().await?;

        if req.logic {
            client
                .execute(
                "update chnots set delete_time = CURRENT_TIMESTAMP where id = $1",
                &[&req.chnot_id],
            )
            .await?;
        } else {
            client
                .execute("delete from chnots where id = $1", &[&req.chnot_id])
                .await?;
        }

        Ok(super::ChnotDeletionRsp {})
    }

    async fn chnot_query(
        &self,
        req: KReq<super::ChnotQueryReq>,
    ) -> AResult<super::ChnotQueryRsp<Vec<ChnotWithRelation>>> {
        let client = self.client().await?;

        let chnot_sql = SqlQuery::new()
            .raw("SELECT DISTINCT ON (c.perm_id)")
            .raw("c.*, ch.prev_id, ch.parent_id, ch.insert_time AS rel_insert_time")
            .raw("from")
            .sub("rs", SqlQuery::new()
                .raw("select ring_id from chnots")
                .wheres(Wheres::And(vec![
                    Wheres::is_null("delete_time"),
                    Wheres::equal("domain", req.domain.clone()),
                    Wheres::if_some(req.query.as_ref(), |t| Wheres::ilike("content", t)),
                ])).raw("group by perm_id")
                .raw("order by MAX(case when pinned then 1 else 0 end) desc, MAX(insert_time) desc"))
            .raw("LEFT JOIN chnots_test c ON rs.ring_id = c.ring_id")
            .raw("LEFT JOIN chnot_hierarchies_tests ch ON c.id = ch.chnot_id")
            .raw("ORDER BY c.perm_id, rel_insert_time DESC")
            .build(&mut ValueType::dollar_number())
            .context("unable to build SqlQuery for query chnots")?;

        let cs = client
            .query(&chnot_sql.seg, to_sql!(chnot_sql.values))
            .await?
            .iter()
            .filter_map(|row| {
                let chnot = map_row_to_chnot(row).ok()?;
                Some(ChnotWithRelation {
                    chnot,
                    prev_id: row.try_get("prev_id").ok()?,
                    parent_id: row.try_get("parent_id").ok()?,
                })
            })
            .collect();

        Ok(ChnotQueryRsp {
            data: cs,
            start_index: req.start_index,
        })
    }

    async fn chnot_update(
        &self,
        req: KReq<super::ChnotUpdateReq>,
    ) -> AResult<super::ChnotUpdateRsp> {
        let client = self.client().await?;

        let su = SimpleUpdater::new("chnots")
            .set_if_some("pinned", req.pinned)
            .set_if_some(
                "archive_time",
                req.archive.map(|_| Local::now().fixed_offset()),
        )
            .filters(Wheres::Equal("id", (&req.chnot_id).into()).into());

        let ss = su.build(ValueType::dollar_number());

        if let Some(ss) = ss {
            client.execute(ss.seg.as_str(), to_sql!(ss.values)).await?;
        }

        Ok(super::ChnotUpdateRsp {})
    }
}

impl BackupTrait for Postgres {
    async fn dump_chnots<F, R1>(&self, row_writer: F) -> EResult
    where
        F: Fn(DumpWrapper<Chnot>) -> R1,
        R1: Future<Output = EResult>,
    {
        let client = self.client().await?;
        let rows = client.query_raw("select * from chnots", NO_PARAMS).await?;
        pin_mut!(rows);

        while let Some(row) = rows.try_next().await? {
            if let Ok(obj) = map_row_to_chnot(&row) {
                row_writer(DumpWrapper::of(obj, 1)).await?;
            }
        }

        Ok(())
    }
}

impl ResourceMapper for Postgres {
    async fn insert_resource(
        &self,
        ori_filename: &str,
        id: String,
        content_type: String,
        domain: Option<String>,
    ) -> AResult<Resource> {
        let stmt = self.pool.get().await?;

        let insert_time = chrono::Utc::now().to_owned();

        stmt.execute(
            "insert into resources(id, domain, ori_filename, content_type, insert_time) values ($1,$2,$3,$4, $5)",
            &[&id, &domain, &ori_filename, &content_type, &insert_time]
        ).await
        .map_err(|e| anyhow::Error::new(e))
        .map(|_| Resource {
            id,
            domain,
            ori_filename: ori_filename.to_string(),
            content_type,
            insert_time,
            delete_time: None,
        })
    }

    async fn query_resource_by_id(&self, id: &str) -> AResult<Resource> {
        let stmt = self.pool.get().await?;
        let row = stmt
            .query_one("select * from resources where id = $1", &[&id])
            .await?;

        Ok(Resource {
            id: row.try_get("id")?,
            domain: row.try_get("domain")?,
            ori_filename: row.try_get("ori_filename")?,
            content_type: row.try_get("content_type")?,
            insert_time: row.try_get("insert_time")?,
            delete_time: row.try_get("delete_time")?,
        })
    }
}

impl<'a> FromSql<'a> for ChnotType {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <&str as tokio_postgres::types::FromSql>::from_sql(ty, raw)
            .and_then(|s| Ok(ChnotType::from_str(s)?))
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        <&str as tokio_postgres::types::FromSql>::accepts(ty)
    }
}

impl ToSql for ChnotType {
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
        }
    }
}
