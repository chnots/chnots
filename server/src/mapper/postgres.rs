use std::{str::FromStr, sync::Arc};

use crate::{
    app::ShareAppState,
    model::v1::db::chnot::{Chnot, ChnotComment, ChnotType},
    utils::sql_param_builder::{self, extract_magic_sql_ph, SqlParamBuilder, MAGIC_SQL_PH},
};
use arc_swap::ArcSwap;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::{DateTime, FixedOffset, Local};
use deadpool_postgres::{Client, Pool};
use postgres_types::{to_sql_checked, FromSql, ToSql};
use serde::Deserialize;
use tokio_postgres::Row;
use tracing::{error, info};
use uuid::Uuid;

use crate::model::v1::dto::*;

use super::{ChnotMapper, Db, TableFounder};

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
    app_state: ArcSwap<Option<ShareAppState>>,
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
        pinned: row.try_get("pinned")?,
        archive_time: row.try_get("archive_time")?,
    };

    Ok(chnot)
}

fn map_row_to_chnot_comment(row: &Row) -> AResult<ChnotComment> {
    let chnot = ChnotComment {
        id: row.try_get("id")?,
        chnot_perm_id: row.try_get("chnot_perm_id")?,
        content: row.try_get("content")?,
        insert_time: row.try_get("insert_time")?,
        delete_time: row.try_get("delete_time")?,
    };

    Ok(chnot)
}

impl Postgres {
    pub fn new(config: PostgresConfig) -> AResult<Postgres> {
        let pool = Into::<deadpool_postgres::Config>::into(config)
            .create_pool(None, tokio_postgres::NoTls)?;

        Ok(Postgres {
            pool,
            app_state: Default::default(),
        })
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

    fn app_state(&self) -> Option<ShareAppState> {
        self.app_state.load().as_ref().clone()
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
    async fn _ensure_table_chnot_comments(&self) -> EResult {
        self.create_table(
            "chnot_comments",
            "create table chnot_comments (
    id VARCHAR(40) NOT NULL,
    chnot_perm_id VARCHAR(40) NOT NULL,

    content TEXT NOT NULL,

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

    async fn _ensure_table_chnots(&self) -> EResult {
        self.create_table(
            "chnots",
            "create table chnots (
    id VARCHAR(40) NOT NULL,
    chnot_perm_id VARCHAR(40) NOT NULL,

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
}

impl ChnotMapper for Postgres {
    async fn chnot_overwrite(
        &self,
        req: ReqWrapper<ChnotInsertionReq>,
    ) -> AResult<ChnotInsertionRsp> {
        let chnot = &req.body.chnot;
        let stmt = self.pool.get().await?;

        stmt.execute("update chnots set delete_time = CURRENT_TIMESTAMP where perm_id = $1 and delete_time is null", &[&chnot.perm_id]).await?;

        stmt.execute(
            "insert into chnots(id, perm_id, pinned, content, type, domain, insert_time, update_time) values($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &chnot.id,
                &chnot.perm_id,
                &chnot.pinned,
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

    async fn chnot_delete(
        &self,
        req: ReqWrapper<super::ChnotDeletionReq>,
    ) -> AResult<super::ChnotDeletionRsp> {
        let stmt = self.pool.get().await?;

        if req.logic {
            stmt.execute(
                "update chnots set delete_time = CURRENT_TIMESTAMP where id = $1",
                &[&req.chnot_id],
            )
            .await?;
        } else {
            stmt.execute("delete from chnots where id = $1", &[&req.chnot_id])
                .await?;
        }

        Ok(super::ChnotDeletionRsp {})
    }

    async fn chnot_query(
        &self,
        req: ReqWrapper<super::ChnotQueryReq>,
    ) -> AResult<super::ChnotQueryRsp> {
        let stmt = self.pool.get().await?;

        let mut sql = "select * from chnots ".to_string();

        let builder = SqlParamBuilder::new();
        let builder = if let Some(query) = &req.query {
            builder.ilike("content", query)
        } else {
            builder
        };

        let (params, values) = builder
            .with(
                "domain",
                self.app_state()
                    .unwrap()
                    .domains
                    .managed(req.domain.as_ref().unwrap().as_str())
                    .to_vec(),
            )
            .fixed(" and delete_time is null ")
            .fixed(format!(
                " order by pinned desc, insert_time desc limit {} offset {}",
                req.page_size, req.start_index
            ))
            .build();

        sql.push_str(" where ");
        sql.push_str(&params);

        let params = values
            .iter()
            .map(|param| &*param as &(dyn ToSql + Sync))
            .collect::<Vec<&(dyn ToSql + Sync)>>();

        let sql = extract_magic_sql_ph(sql.as_str());

        let chnots: Vec<Chnot> = stmt
            .query(sql.as_str(), &params)
            .await?
            .into_iter()
            .filter_map(|e| chnot_row_to_obj(&e).ok())
            .collect();

        let sql = format!(
            "select * from chnot_comments where chnot_perm_id in ({}) order by insert_time asc",
            chnots
                .iter()
                .map(|e| format!("'{}'", e.perm_id))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let comments: Vec<ChnotComment> = stmt
            .query(sql.as_str(), &[])
            .await?
            .iter()
            .filter_map(|row| {
                map_row_to_chnot_comment(row)
                    .map_err(|err| error!("unable to map: {}", err))
                    .ok()
            })
            .collect();

        let mut chnot_with_comments = vec![];

        for chnot in chnots.into_iter() {
            let comments = comments
                .iter()
                .filter(|c| c.chnot_perm_id == chnot.perm_id)
                .map(|c| c.clone())
                .collect();
            let chnot_with_comment = ChnotWithComment { chnot, comments };
            chnot_with_comments.push(chnot_with_comment)
        }

        Ok(super::ChnotQueryRsp {
            has_more: chnot_with_comments.len() >= req.page_size.try_into().unwrap(),
            data: chnot_with_comments,
            next_start: req.start_index.saturating_add(req.page_size),
            this_start: req.start_index,
        })
    }

    async fn chnot_update(
        &self,
        req: ReqWrapper<super::ChnotUpdateReq>,
    ) -> AResult<super::ChnotUpdateRsp> {
        let stmt = self.pool.get().await?;

        let mut seg = String::new();
        let mut args: Vec<Box<dyn ToSql + Sync + Send>> = vec![];
        let mut init = true;
        if let Some(pinned) = req.pinned {
            if !init {
                seg.push(',');
            } else {
                init = false;
            }
            seg.push_str("pinned = ");
            seg.push_str(&MAGIC_SQL_PH);
            args.push(Box::new(pinned));
        }

        if let Some(archive) = req.archive {
            if !init {
                seg.push(',');
            } else {
                init = false;
            }
            seg.push_str("archive_time = ");
            seg.push_str(&MAGIC_SQL_PH);
            if archive {
                args.push(Box::new(Local::now()));
            } else {
                args.push(Box::new(None::<DateTime<FixedOffset>>));
            }
        }

        let sql = sql_param_builder::extract_magic_sql_ph(
            format!("update chnots set {} where id = {}", seg, MAGIC_SQL_PH).as_str(),
        );
        args.push(Box::new(req.chnot_id.to_string()));

        let params = args
            .iter()
            .map(|param| param.as_ref())
            .collect::<Vec<&(dyn ToSql + Sync + Send)>>();

        stmt.execute(
            sql.as_str(),
            params
                .iter()
                .map(|e| *e as &(dyn ToSql + Sync))
                .collect::<Vec<&(dyn ToSql + Sync)>>()
                .as_slice(),
        )
        .await?;

        Ok(super::ChnotUpdateRsp {})
    }

    async fn chnot_comment_add(
        &self,
        req: ReqWrapper<ChnotCommentAddReq>,
    ) -> AResult<ChnotCommentAddRsp> {
        let stmt = self.pool.get().await?;

        stmt.execute(
            "insert into chnot_comments(id, chnot_perm_id, content, insert_time) values($1,$2,$3,$4)",
            &[
                &Uuid::new_v4().to_string(),
                &req.chnot_perm_id,
                &req.content,
                &req.insert_time,
            ],
        )
        .await?;

        Ok(ChnotCommentAddRsp {})
    }

    async fn chnot_comment_delete(
        &self,
        req: ReqWrapper<ChnotCommentDeleteReq>,
    ) -> AResult<ChnotCommentDeleteRsp> {
        let stmt = self.pool.get().await?;

        if req.logic {
            stmt.execute(
                "update chnot_comments set delete_time = CURRENT_TIMESTAMP where id = $1",
                &[&req.id],
            )
            .await?;
        } else {
            stmt.execute("delete from chnot_comments where id = $1", &[&req.id])
                .await?;
        }

        Ok(super::ChnotCommentDeleteRsp {})
    }
}

impl Db for Postgres {
    fn set_app_state(&self, state: ShareAppState) {
        self.app_state.store(Arc::new(Some(state)))
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
