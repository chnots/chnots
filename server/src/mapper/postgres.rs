use std::str::FromStr;

use crate::model::chnot::{Chnot, ChnotType};
use chin_tools::wrapper::anyhow::{AResult, EResult};
use deadpool_postgres::{Client, Pool};
use postgres_types::{to_sql_checked, FromSql, ToSql};
use serde::Deserialize;
use tokio_postgres::Row;
use tracing::info;

use super::{ChnotInsertionReq, ChnotInsertionRsp, ChnotMapper, Db, TableFounder};

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

fn chnot_row_to_obj(row: &Row) -> AResult<Chnot> {
    let chnot = Chnot {
        id: row.try_get("id")?,
        perm_id: row.try_get("perm_id")?,
        content: row.try_get("content")?,
        r#type: row.try_get("type")?,
        domain: row.try_get("domain")?,
        delete_time: row.try_get("delete_time")?,
        insert_time: row.try_get("insert_time")?,
        update_time: row.try_get("update_time")?,
    };

    Ok(chnot)
}

impl Postgres {
    pub fn new(config: PostgresConfig) -> AResult<Postgres> {
        let pool = Into::<deadpool_postgres::Config>::into(config)
            .create_pool(None, tokio_postgres::NoTls)?;

        Ok(Postgres { pool })
    }

    async fn get_client(&self) -> AResult<Client> {
        self.pool.get().await.map_err(anyhow::Error::new)
    }

    async fn check_if_table_not_exists(&self, table_name: &str) -> AResult<bool> {
        let client = self.get_client().await?;
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
            let client = self.get_client().await?;
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

    content TEXT NOT NULL,
    type VARCHAR(255) NOT NULL,
    domain TEXT NOT NULL,

    delete_time timestamptz DEFAULT NULL,
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    update_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
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
}

impl ChnotMapper for Postgres {
    async fn chnot_overwrite(&self, req: ChnotInsertionReq) -> AResult<ChnotInsertionRsp> {
        let ChnotInsertionReq { chnot } = req;
        let stmt = self.pool.get().await?;

        stmt.execute("update chnots set delete_time = CURRENT_TIMESTAMP where perm_id = $1 and delete_time is null", &[&chnot.perm_id]).await?;

        stmt.execute(
            "insert into chnots(id, perm_id, content, type, domain, insert_time, update_time) values($1, $2, $3, $4, $5, $6, $7)",
            &[
                &chnot.id,
                &chnot.perm_id,
                &chnot.content,
                &chnot.r#type,
                &chnot.domain,
                &chnot.insert_time,
                &chnot.update_time,
            ],
        )
        .await?;

        Ok(ChnotInsertionRsp {})
    }

    async fn chnot_delete(&self, req: super::ChnotDeletionReq) -> AResult<super::ChnotDeletionRsp> {
        let stmt = self.pool.get().await?;

        if req.logic {
            stmt.execute(
                "update chnots set delete_time = CURRENT_TIMESTAMP where id = ?",
                &[&req.chnot_id],
            )
            .await?;
        } else {
            stmt.execute("delete from chnots where id = $1", &[&req.chnot_id])
                .await?;
        }

        Ok(super::ChnotDeletionRsp {})
    }

    async fn chnot_query(&self, req: super::ChnotQueryReq) -> AResult<super::ChnotQueryRsp> {
        let stmt = self.pool.get().await?;

        let col = match req.query.as_ref() {
            Some(query) => 
                stmt
            .query(
                "select * from chnots where content ilike $1 order by insert_time desc limit $2 offset $3",
                &[&format!("%{}%", query), &req.page_size, &req.start_index],
            ).await?
            .iter()
            .filter_map(|row| chnot_row_to_obj(row).ok())
            .collect()
            ,
            None => 
                stmt
            .query(
                "select * from chnots order by insert_time desc limit $1 offset $2",
                &[&req.page_size, &req.start_index],
            ).await?
            .iter()
            .filter_map(|row| chnot_row_to_obj(row).ok())
            .collect()  
        };
            

        Ok(super::ChnotQueryRsp { data: col, next_start: req.start_index.saturating_add(req.page_size), this_start: req.start_index })
    }
}

impl Db for Postgres {}

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
