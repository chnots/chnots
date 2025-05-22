use std::ops::Deref;

use crate::{
    mapper::{
        db::{
            chnot::{chnot_query_mapper, chnot_query_sql},
            kdb::KDbConnBehaiverSync,
            KDb, KDbBehaiver, KDbConnBehaiver, KDbRow, KDbRowBehavier,
        },
        ChnotMapper,
    },
    model::{
        db::chnot::*,
        dto::{
            chnot::{ChnotOverwriteReq, ChnotOverwriteRsp},
            KReq,
        },
    },
};
use anyhow::anyhow;
use chin_sql::Wheres;
use chin_sql::{SqlInserter, SqlReader, SqlUpdater};
use chin_tools::{utils::id_util, AResult, EResult, SharedStr};

use chrono::{DateTime, FixedOffset, TimeDelta};

#[derive(Clone)]
enum MetaId {
    Old(SharedStr),
    New(SharedStr),
}

impl Deref for MetaId {
    type Target = SharedStr;

    fn deref(&self) -> &Self::Target {
        match self {
            MetaId::Old(s) => s,
            MetaId::New(s) => s,
        }
    }
}

impl MetaId {
    fn to_str(&self) -> SharedStr {
        SharedStr::new(self.deref().clone())
    }
}

struct OldInfo {
    id: String,
    content: String,
    insert_time: DateTime<FixedOffset>,
}
fn to_old_info(row: KDbRow<'_>) -> AResult<OldInfo> {
    Ok(OldInfo {
        id: row.try_get("id")?,
        content: row.try_get("content")?,
        insert_time: row.try_get("insert_time")?,
    })
}

macro_rules! handle_insert {
    ( $tx:expr, $meta_id:expr, $req:expr $(, $wait:tt)?) => {
        let insert_rec = |id| SqlInserter::new(ChnotRecord::TABLE)
            .fields(ChnotRecord::ID, id)
            .fields(ChnotRecord::INSERT_TIME, &$req.insert_time)
            .fields(ChnotRecord::META_ID, (*$meta_id).clone())
            .fields(ChnotRecord::CONTENT, &$req.content);
        // SQL operations
        let insert_meta = SqlInserter::new(ChnotMetadata::TABLE)
            .fields(ChnotMetadata::ID, (*$meta_id).clone())
            .fields(ChnotMetadata::INSERT_TIME, &$req.insert_time)
            .fields(ChnotMetadata::WORKSPACE, &$req.workspace)
            .fields(ChnotMetadata::KIND, $req.kind.as_ref());
        let update_meta_utime = SqlUpdater::new(ChnotMetadata::TABLE)
            .set(ChnotMetadata::UPDATE_TIME, &$req.insert_time)
            .r#where(Wheres::equal(
                ChnotMetadata::ID,
                (*$meta_id).clone(),
            ));
        match $meta_id {
            MetaId::Old(_) => {
                // Query for existing record
                let query_old_rec = SqlReader::read(ChnotRecord::TABLE,
                    &[ChnotRecord::ID, ChnotRecord::CONTENT, ChnotRecord::INSERT_TIME]
                )
                    .r#where(Wheres::and([
                        Wheres::equal(ChnotRecord::META_ID, (*$meta_id).clone()),
                        Wheres::is_null(ChnotRecord::OMIT_TIME),
                    ]));

                let update_rec = |id| {
                    SqlUpdater::new(ChnotRecord::TABLE)
                        .set(ChnotRecord::CONTENT, &$req.content)
                        .set(ChnotRecord::INSERT_TIME, &$req.insert_time)
                        .r#where(Wheres::equal(ChnotRecord::ID, id))
                };

                let update_omit = SqlUpdater::new(ChnotRecord::TABLE)
                    .set(ChnotRecord::OMIT_TIME, &$req.insert_time)
                    .r#where(Wheres::equal(ChnotRecord::META_ID, $meta_id.to_str()));

                // Get old record info
                let old_info = $tx.qry_opt(query_old_rec, |e| to_old_info(e))$(.$wait)??;
                let Some(OldInfo {
                    id: old_id,
                    content: old_cont,
                    insert_time: old_time,
                }) = old_info else {
                    return Err(anyhow::anyhow!("Old Chnot is absent"));
                };

                // Determine update strategy
                let should_update =
                    textdistance::str::sift4_simple(&old_cont, &$req.content) <= 50
                    && $req.insert_time.signed_duration_since(old_time).abs() < TimeDelta::hours(1);

                if should_update {
                    $tx.exec_and_check(update_rec(old_id), |c| c == 1)$(.$wait)??;
                } else {
                    $tx.exec(update_omit)$(.$wait)??;
                    $tx.exec(insert_rec(id_util::generate_uuid()))$(.$wait)??;
                }
            }
            MetaId::New(_) => {
                $tx.exec(insert_rec(id_util::generate_uuid()))$(.$wait)??;
                $tx.exec(insert_meta)$(.$wait)??;
            }
        }

        // Final update to metadata timestamp
        $tx.exec_and_check(update_meta_utime, |c| c == 1)$(.$wait)??;

        $tx.commit()$(.$wait)??;
    };
}

impl KDb {
    pub async fn chnot_overwrite(
        &self,
        req: KReq<ChnotOverwriteReq>,
    ) -> AResult<ChnotOverwriteRsp> {
        tracing::debug!("begin to overwrite chnot, {:?}", req.meta_id);

        let meta_id = match &req.meta_id {
            Some(id) => MetaId::Old(SharedStr::new(id)),
            None => MetaId::New(SharedStr::new(id_util::generate_uuid())),
        };

        // 1. Query chnot by meta_id.
        // 2. If chnot is existed.
        // 3. Compare new and old, if we could just update it, update.
        match self.conn().await? {
            crate::mapper::db::KDbConn::Sqlite(db) => {
                let meta_id = meta_id.to_owned();
                let ins: Result<EResult, deadpool_sqlite::InteractError> = db
                    .pool()
                    .get()
                    .await?
                    .interact(move |conn| {
                        let tx = conn.transaction().unwrap(); // TODO
                        handle_insert!(tx, meta_id, req);
                        Ok(())
                    })
                    .await;
                ins.map_err(|e| anyhow!(e.to_string()))??;
            }
            crate::mapper::db::KDbConn::Postgres(mut db) => {
                let tx = db.build_transaction().start().await?;
                handle_insert!(tx, meta_id, req, await);
            }
        }

        let query_sql = chnot_query_sql().r#where(Wheres::and([
            Wheres::is_null("r.omit_time"),
            Wheres::equal("m.id", meta_id.as_str()),
        ]));
        let chnot = self
            .conn()
            .await?
            .qry_one(query_sql, chnot_query_mapper, true)
            .await?;

        self.chnot_tag_update_single_chnot(
            &chnot.record.content,
            &chnot.meta.id,
            &chnot.meta.workspace,
        )
        .await?;

        Ok(ChnotOverwriteRsp { chnot: chnot })
    }
}
