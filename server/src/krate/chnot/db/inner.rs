use std::ops::Deref;

use super::*;
use crate::mapper::db::{KDbExecutor, KDbExecutorBehaiver, KDbRow, KDbTx};
use crate::model::dto::KReq;
use chin_sql::{SqlDeleter, Wheres};
use chin_sql::{SqlInserter, SqlReader, SqlUpdater};
use chin_tools::utils::id_util::generate_uuid;
use chin_tools::{utils::id_util, AResult, SharedStr};

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

struct OldInfo {
    id: String,
    content: String,
    insert_time: DateTime<FixedOffset>,
}
fn to_old_info(row: KDbRow) -> AResult<OldInfo> {
    Ok(OldInfo {
        id: row.try_get("id")?,
        content: row.try_get("content")?,
        insert_time: row.try_get("insert_time")?,
    })
}

// TODO: We could not use `impl<'a, T: KDbExecutorBehavier> for KImplWrapper<T>` here, because 
// implementation of std::marker::Send is not general enough will be raised.
// Waiting for https://github.com/rust-lang/rust/issues/110338
impl<'a> KDbExecutor<'a> {
    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult {
        self.exec(
            SqlInserter::new(ChnotTag::TABLE)
                .field(ChnotTag::ID, req.id)
                .field(ChnotTag::CHNOT_META_ID, req.chnot_meta_id)
                .field(ChnotTag::KSPACE, req.kspace)
                .field(ChnotTag::TAG, req.tag)
                .field(ChnotTag::CATEGORY, req.category)
                .field(ChnotTag::INSERT_TIME, req.insert_time),
        )
        .await?;

        Ok(())
    }
}

impl<'a> KDbTx<'a> {
    pub(super) async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        for e in chnot_meta_ids {
            self.exec(
                SqlDeleter::new(ChnotTag::TABLE).r#where(Wheres::equal(ChnotTag::CHNOT_META_ID, e)),
            )
            .await?;
        }

        Ok(())
    }

    pub(super) async fn chnot_tag_update_single_chnot(&self, req: ChnotTagUpdateReq) -> EResult {
        let ChnotTagUpdateReq {
            content,
            meta_id,
            kspace,
        } = req;
        self.chnot_tag_delete(vec![&meta_id]).await?;

        let tags = get_hashtags(&content);
        let parent_tags: Vec<&str> = tags
            .iter()
            .flat_map(|tag| {
                let mut more = vec![];
                for (size, c) in tag.char_indices() {
                    if c == '/' {
                        more.push(&tag[..size]);
                    }
                }
                more
            })
            .unique()
            .collect();

        let executor = KDbExecutor::Tx(self);
        if parent_tags.is_empty() {
            executor.chnot_tag_insert(ChnotTag {
                id: id_util::generate_uuid(),
                kspace: kspace.to_owned(),
                tag: UNTAGGED_TAG.to_owned(),
                chnot_meta_id: meta_id.to_string(),
                insert_time: Utc::now().fixed_offset(),
                category: ChnotTagType::Dir,
            })
            .await?;
        }
        for tag in parent_tags {
            executor.chnot_tag_insert(ChnotTag {
                id: id_util::generate_uuid(),
                kspace: kspace.to_owned(),
                tag: tag.to_owned(),
                chnot_meta_id: meta_id.to_string(),
                insert_time: Utc::now().fixed_offset(),
                category: ChnotTagType::ParentDir,
            })
            .await?;
        }

        for tag in tags {
            executor.chnot_tag_insert(ChnotTag {
                id: id_util::generate_uuid(),
                kspace: kspace.to_owned(),
                tag: tag.to_owned(),
                chnot_meta_id: meta_id.to_string(),
                insert_time: Utc::now().fixed_offset(),
                category: ChnotTagType::Dir,
            })
            .await?;
        }

        Ok(())
    }

    pub(super) async fn chnot_overwrite(
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

        let insert_rec = |id| {
            SqlInserter::new(ChnotRecord::TABLE)
                .field(ChnotRecord::ID, id)
                .field(ChnotRecord::INSERT_TIME, req.insert_time)
                .field(ChnotRecord::META_ID, (*meta_id).to_string())
                .field(ChnotRecord::CONTENT, &req.content)
        };
        // SQL operations
        let insert_meta = SqlInserter::new(ChnotMetadata::TABLE)
            .field(ChnotMetadata::ID, (*meta_id).to_string())
            .field(ChnotMetadata::INSERT_TIME, req.insert_time)
            .field(ChnotMetadata::KSPACE, &req.kspace)
            .field(ChnotMetadata::KIND, req.kind.as_ref());
        let update_meta_utime = SqlUpdater::new(ChnotMetadata::TABLE)
            .set(ChnotMetadata::UPDATE_TIME, req.insert_time)
            .r#where(Wheres::equal(ChnotMetadata::ID, (*meta_id).to_string()));

        let rec_id = generate_uuid();
        match meta_id {
            MetaId::Old(_) => {
                // Query for existing record
                let query_old_rec = SqlReader::read(
                    ChnotRecord::TABLE,
                    &[
                        ChnotRecord::ID,
                        ChnotRecord::CONTENT,
                        ChnotRecord::INSERT_TIME,
                    ],
                )
                .r#where(Wheres::and([
                    Wheres::equal(ChnotRecord::META_ID, (*meta_id).to_string()),
                    Wheres::is_null(ChnotRecord::OMIT_TIME),
                ]));

                let update_rec = |id| {
                    SqlUpdater::new(ChnotRecord::TABLE)
                        .set(ChnotRecord::CONTENT, &req.content)
                        .set(ChnotRecord::INSERT_TIME, req.insert_time)
                        .r#where(Wheres::equal(ChnotRecord::ID, id))
                };

                let update_omit = SqlUpdater::new(ChnotRecord::TABLE)
                    .set(ChnotRecord::OMIT_TIME, req.insert_time)
                    .r#where(Wheres::equal(ChnotRecord::META_ID, meta_id.to_string()));

                // Get old record info
                let old_info = self.qry_opt(query_old_rec, to_old_info).await?;
                let Some(OldInfo {
                    id: old_id,
                    content: old_cont,
                    insert_time: old_time,
                }) = old_info
                else {
                    return Err(anyhow::anyhow!("Old Chnot is absent"));
                };

                // Determine update strategy
                let should_update = textdistance::str::sift4_simple(&old_cont, &req.content) <= 50
                    && req.insert_time.signed_duration_since(old_time).abs() < TimeDelta::hours(1);

                if should_update {
                    self.exec_and_check(update_rec(old_id), |c| c == 1).await?;
                } else {
                    self.exec(update_omit).await?;
                    self.exec(insert_rec(rec_id.clone())).await?;
                }
            }
            MetaId::New(_) => {
                self.exec(insert_rec(rec_id.clone())).await?;
                self.exec(insert_meta).await?;
            }
        }

        // Final update to metadata timestamp
        self.exec_and_check(update_meta_utime, |c| c == 1).await?;

        self.chnot_tag_update_single_chnot(ChnotTagUpdateReq {
            content: req.content.clone(),
            meta_id: meta_id.to_string(),
            kspace: req.kspace,
        })
        .await?;

        let query_sql = chnot_query_sql().r#where(Wheres::and([
            Wheres::is_null("r.omit_time"),
            Wheres::equal("m.id", meta_id.as_str()),
        ]));
        let chnot = self.qry_one(query_sql, chnot_query_mapper, true).await?;

        Ok(ChnotOverwriteRsp { chnot })
    }
}
