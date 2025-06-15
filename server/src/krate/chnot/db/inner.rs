use std::ops::Deref;

use super::*;
use crate::mapper::db::{KDbExecutor, KDbExecutorBehaiver, KDbRow, KDbTx};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;
use chin_sql::{SqlDeleter, Wheres};
use chin_sql::{SqlBuilder, SqlUpdater};
use chin_sql::time_type::TID;
use chin_tools::AResult;

use chrono::TimeDelta;

#[derive(Clone)]
enum MetaId {
    Old(TID),
    New(TID),
}

impl Deref for MetaId {
    type Target = TID;

    fn deref(&self) -> &Self::Target {
        match self {
            MetaId::Old(s) => s,
            MetaId::New(s) => s,
        }
    }
}

struct OldInfo {
    tid: TID,
    content: String,
}
fn to_old_info(row: KDbRow) -> AResult<OldInfo> {
    Ok(OldInfo {
        tid: row.try_get("meta_tid")?,
        content: row.try_get("content")?,
    })
}

// TODO: We could not use `impl<'a, T: KDbExecutorBehavier> for KImplWrapper<T>` here, because
// implementation of std::marker::Send is not general enough will be raised.
// Waiting for https://github.com/rust-lang/rust/issues/110338
impl<'a> KDbExecutor<'a> {
    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult {
        self.exec(req.to_sql_inserter()).await?;

        Ok(())
    }

    async fn chnot_record_insert(&self, req: ChnotRecord) -> EResult {
        self.exec(req.to_sql_inserter()).await?;

        Ok(())
    }

    async fn chnot_meta_insert(&self, req: ChnotMetadata) -> EResult {
        self.exec(req.to_sql_inserter()).await?;

        Ok(())
    }
}

impl<'a> KDbTx<'a> {
    pub(super) async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<TID>) -> EResult {
        for e in chnot_meta_ids {
            self.exec(
                SqlDeleter::new(ChnotTag::TABLE).r#where(Wheres::equal(ChnotTag::META_TID, e)),
            )
            .await?;
        }

        Ok(())
    }

    pub(super) async fn chnot_tag_update_single_chnot(&self, req: ChnotTagUpdateReq) -> EResult {
        let ChnotTagUpdateReq {
            content,
            meta_tid: meta_id,
            kspace,
        } = req;
        self.chnot_tag_delete(vec![meta_id]).await?;

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
            executor
                .chnot_tag_insert(ChnotTag {
                    tid: TID::default(),
                    kspace: kspace.to_owned(),
                    tag: UNTAGGED_TAG.to_owned(),
                    meta_tid: meta_id,
                    category: ChnotTagType::Dir,
                    omit_tid: OmitTID::never(),
                })
                .await?;
        }
        for tag in parent_tags {
            executor
                .chnot_tag_insert(ChnotTag {
                    tid: TID::default(),
                    kspace: kspace.to_owned(),
                    tag: tag.to_owned(),
                    meta_tid: meta_id,
                    category: ChnotTagType::ParentDir,
                    omit_tid: OmitTID::never(),
                })
                .await?;
        }

        for tag in tags {
            executor
                .chnot_tag_insert(ChnotTag {
                    tid: TID::default(),
                    kspace: kspace.to_owned(),
                    tag: tag.to_owned(),
                    meta_tid: meta_id,
                    category: ChnotTagType::Dir,
                    omit_tid: OmitTID::never(),
                })
                .await?;
        }

        Ok(())
    }

    pub(super) async fn chnot_overwrite(
        &self,
        req: KReq<ChnotOverwriteReq>,
    ) -> AResult<ChnotOverwriteRsp> {
        tracing::debug!("begin to overwrite chnot, {:?}", req.meta_tid);

        let meta_tid = match req.meta_tid {
            Some(tid) => MetaId::Old(tid),
            None => MetaId::New(TID::default()),
        };
        let archor: bool;

        let rec_tid = TID::default();
        match meta_tid {
            MetaId::Old(meta_tid) => {
                // Query for existing record
                let query_old_rec = SqlBuilder::read(
                    ChnotRecord::TABLE,
                    &[ChnotRecord::META_TID, ChnotRecord::CONTENT],
                )
                .r#where(Wheres::and([
                    Wheres::equal(ChnotRecord::META_TID, meta_tid),
                    Wheres::equal(ChnotRecord::OMIT_TID, OmitTID::never()),
                ]));

                // Get old record info
                let Some(OldInfo {
                    tid: old_id,
                    content: old_cont,
                }) = self.qry_opt(query_old_rec, to_old_info).await?
                else {
                    return Err(anyhow::anyhow!("Old Chnot is absent"));
                };

                // Determine update strategy
                archor = !textdistance::str::sift4_simple(&old_cont, &req.content) <= 50
                    && rec_tid
                        .as_utc()
                        .signed_duration_since(old_id.as_utc())
                        .abs()
                        < TimeDelta::hours(1);

                let rec = ChnotRecord {
                    tid: rec_tid,
                    meta_tid,
                    omit_tid: OmitTID::never(),
                    content: req.content.clone(),
                    archor,
                };

                if archor {
                    let update_rec = |tid| {
                        SqlUpdater::new(ChnotRecord::TABLE)
                            .set(ChnotRecord::CONTENT, &req.content)
                            .r#where(Wheres::equal(ChnotRecord::META_TID, tid))
                    };
                    self.exec_and_check(update_rec(old_id), |c| c == 1).await?;
                } else {
                    let update_omit = SqlUpdater::new(ChnotRecord::TABLE)
                        .set(ChnotRecord::OMIT_TID, OmitTID::now())
                        .r#where(Wheres::equal(ChnotRecord::META_TID, meta_tid));
                    self.exec(update_omit).await?;
                    self.as_executor().chnot_record_insert(rec).await?;
                }
            }
            MetaId::New(meta_tid) => {
                archor = true;
                let rec = ChnotRecord {
                    tid: rec_tid,
                    meta_tid,
                    omit_tid: OmitTID::never(),
                    content: req.content.clone(),
                    archor: true,
                };
                self.as_executor().chnot_record_insert(rec).await?;
                let meta = ChnotMetadata {
                    tid: meta_tid,
                    kspace: req.kspace.clone(),
                    kind: req.kind.to_string(),
                    pin_time: None,
                    omit_tid: OmitTID::never(),
                    archive_time: None,
                };
                self.as_executor().chnot_meta_insert(meta).await?;
            }
        }
        if let Some(kid) = req.kind_id.clone() {
            self.as_executor()
                .exec(ChnotKindId {
                    meta_tid: *meta_tid,
                    omit_tid: OmitTID::never(),
                    kind_id: kid,
                }.to_sql_inserter())
                .await?;
        }

        self.chnot_tag_update_single_chnot(ChnotTagUpdateReq {
            content: req.content.clone(),
            meta_tid: *meta_tid,
            kspace: req.kspace.clone(),
        })
        .await?;

        Ok(ChnotOverwriteRsp {
            meta_tid: *meta_tid,
            rec_tid,
            ksapce: req.kspace,
            archor,
        })
    }
}
