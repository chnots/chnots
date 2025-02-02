use std::str::FromStr;

use crate::{
    mapper::ChnotMapper,
    model::db::chnot::{ChnotKind, ChnotMetadata, ChnotRecord},
    to_sql,
    util::sql_builder::{LimitOffset, PlaceHolderType, SqlSegBuilder, SqlUpdater, Wheres},
};
use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::Local;
use postgres_types::{to_sql_checked, FromSql, ToSql};
use tokio_postgres::Row;
use tracing::info;
use tracing_log::log;

use crate::model::dto::chnot::*;

use super::{KReq, Postgres};

impl<'a> FromSql<'a> for ChnotKind {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        <&str as tokio_postgres::types::FromSql>::from_sql(ty, raw)
            .and_then(|s| Ok(ChnotKind::from_str(s)?))
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        <&str as tokio_postgres::types::FromSql>::accepts(ty)
    }
}

impl ToSql for ChnotKind {
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

fn map_row_to_chnot_record(row: &Row) -> AResult<ChnotRecord> {
    let chnot = ChnotRecord {
        id: row.try_get("id")?,
        meta_id: row.try_get("meta_id")?,
        content: row.try_get("content")?,
        omit_time: row.try_get("delete_time")?,
        insert_time: row.try_get("insert_time")?,
    };

    Ok(chnot)
}

fn map_row_to_chnot_metadata(row: &Row) -> AResult<ChnotMetadata> {
    let header = ChnotMetadata {
        id: row.try_get("id")?,
        insert_time: row.try_get("insert_time")?,
        delete_time: row.try_get("delete_time")?,
        update_time: row.try_get("update_time")?,
        namespace: row.try_get("namespace")?,
        pin_time: row.try_get("pin_time")?,
        kind: row.try_get("kind")?,
    };

    Ok(header)
}

impl ChnotMapper for Postgres {
    async fn ensure_table_chnot_record(&self) -> EResult {
        self.create_table(
            "create table IF NOT EXISTS chnot_record (
    id VARCHAR(40) NOT NULL,
    meta_id VARCHAR(40) NOT NULL,
    content TEXT NOT NULL,
    omit_time timestamptz DEFAULT NULL,
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    primary key (id)
)",
        )
        .await
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        self.create_table(
            "create table IF NOT EXISTS chnot_metadata (
    id VARCHAR(40) NOT NULL,
    namespace VARCHAR(100) NOT NULL,
    kind VARCHAR(100) NOT NULL,
    pin_time timestamptz DEFAULT NULL,
    delete_time timestamptz DEFAULT NULL,    
    update_time timestamptz DEFAULT NULL,
    insert_time timestamptz NOT NULL default CURRENT_TIMESTAMP,
    primary key (id)
)",
        )
        .await
    }

    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        let chnot = &req.body.chnot;

        let mut client = self.client().await?;

        let transaction = client.build_transaction().start().await?;

        let old_id = transaction
            .query_opt(
                "select id from chnot_record where meta_id = $1 and omit_time is null",
                &[&req.chnot.meta_id],
            )
            .await?
            .and_then(|row| {
                let id: String = row.try_get("id").ok()?;
                Some(id)
            });

        if let Some(old_id) = old_id.as_ref() {
            transaction
                .execute(
                    "update chnot_record set omit_time = $1 where id = $2",
                    &[&chnot.insert_time, &old_id],
                )
                .await?;
        }

        transaction.execute(
            "insert into chnot_metadata(id, insert_time, namespace, kind) values($1, $2, $3, $4) on CONFLICT (id) DO UPDATE SET update_time = $2",
            &[
                &chnot.meta_id,
                &chnot.insert_time,
                &req.namespace,
                &req.kind
            ]
        ).await?;

        transaction.execute(
            "insert into chnot_record(id, meta_id, content, insert_time) values($1, $2, $3, $4)",
            &[
                &chnot.id,
                &chnot.meta_id,
                &chnot.content,
                &chnot.insert_time
            ]
        ).await?;

        transaction.commit().await?;

        Ok(ChnotOverwriteRsp {
            chnot: Chnot {
                meta: ChnotMetadata {
                    id: chnot.meta_id.clone(),
                    namespace: req.namespace.clone(),
                    kind: req.kind.to_string(),
                    pin_time: None,
                    delete_time: None,
                    update_time: None,
                    insert_time: req.insert_time,
                },
                record: req.chnot.clone(),
            },
        })
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        let client = self.client().await?;

        client
            .execute(
                "update chnot_metadata set delete_time = CURRENT_TIMESTAMP where id = $1",
                &[&req.chnot_id],
            )
            .await?;

        Ok(ChnotDeletionRsp {})
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        let client = self.client().await?;

        let chnot_sql = SqlSegBuilder::new()
            .raw("SELECT r.id as rid, r.content, r.omit_time, r.insert_time as version_time,")
            .raw("m.id as mid, m.namespace, m.kind, m.pin_time, m.delete_time, m.update_time, m.insert_time as init_time")
            .raw("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_id = m.id")
            .r#where(Wheres::and(
                [
                    // default without deleted chnot
                    Wheres::transform(req.with_deleted, |e| {
                        if e.unwrap_or(false) {
                            Wheres::none()
                        } else {
                            Wheres::is_null("delete_time")
                        }
                    }),
                    // default without omit chnot record
                    // TODO: group by perm id
                    Wheres::transform(req.with_omitted, |e| {
                        if e.unwrap_or(false) {
                            Wheres::none()
                        } else {
                            Wheres::is_null("omit_time")
                        }
                    }),
                    Wheres::equal("namespace", req.namespace.clone()),
                    Wheres::if_some(req.query.as_ref(), |content| {
                        Wheres::ilike("content", content)
                    }),
                ]
            ))
            .raw("ORDER BY m.pin_time DESC, r.insert_time desc")
            .custom(
                LimitOffset::new(req.page_size).offset_if_some(Some(req.start_index)).to_box()
            )
            .build(&mut PlaceHolderType::DollarNumber(0))
            .expect("error occured when build sql");
        info!("sql is {}", chnot_sql.seg);

        let cs = client
            .query(&chnot_sql.seg, to_sql!(chnot_sql.values))
            .await?
            .iter()
            .map(|row| {
                let record = ChnotRecord {
                    id: row.try_get("rid")?,
                    meta_id: row.try_get("mid")?,
                    content: row.try_get("content")?,
                    omit_time: row.try_get("omit_time")?,
                    insert_time: row.try_get("version_time")?,
                };
                let meta = ChnotMetadata {
                    id: row.try_get("mid")?,
                    namespace: row.try_get("namespace")?,
                    kind: row.try_get("kind")?,
                    pin_time: row.try_get("pin_time")?,
                    delete_time: row.try_get("delete_time")?,
                    update_time: row.try_get("update_time")?,
                    insert_time: row.try_get("init_time")?,
                };
                Ok(Chnot { record, meta })
            })
            .filter_map(|e: AResult<Chnot>| {
                if e.is_err() {
                    log::error!("unable to remap: {:?}", e.err());
                    None
                } else {
                    e.ok()
                }
            })
            .collect();

        Ok(ChnotQueryRsp {
            data: cs,
            start_index: req.start_index,
        })
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        let client = self.client().await?;

        let su = SqlUpdater::new("chnot_metadata")
            .set_if_some("pinned", req.pinned)
            .set_if_some(
                "archive_time",
                req.archive.map(|_| Local::now().fixed_offset()),
            )
            .r#where(Wheres::equal("id", &req.chnot_meta_id).into());

        let ss = su.build(PlaceHolderType::dollar_number());

        if let Some(ss) = ss {
            client.execute(ss.seg.as_str(), to_sql!(ss.values)).await?;
        }

        Ok(ChnotUpdateRsp {})
    }
}
