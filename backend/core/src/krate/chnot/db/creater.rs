use std::ops::Deref;

use super::*;
use crate::krate::chnot::parser::ChnotParser;
use crate::mapper::db::{KDbExecutor, KDbExecutorBehaiver, KDbRow, KDbTx};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;
use chin_sql::time_type::TID;
use chin_sql::{ChinSqlError, Wheres};
use chin_sql::{SqlBuilder, SqlUpdater};
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
        tid: row.try_get("meta_otid")?,
        content: row.try_get("content")?,
    })
}

// TODO: We could not use `impl<'a, T: KDbExecutorBehavier> for KImplWrapper<T>` here, because
// implementation of std::marker::Send is not general enough will be raised.
// Waiting for https://github.com/rust-lang/rust/issues/110338
impl<'a> KDbExecutor<'a> {
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
    pub(super) async fn chnot_tag_update_single_chnot(
        &self,
        req: ChnotTagUpdateReq,
        chnot_parser: &ChnotParser<'_>,
    ) -> EResult {
        let ChnotTagUpdateReq {
            content: _,
            meta_otid,
            kspace,
        } = req;

        let tags: Result<Vec<Varchar<800>>, ChinSqlError> = chnot_parser
            .get_all_tags()
            .iter()
            .map(|t| t.to_string().try_into())
            .collect();
        let mut tags = tags?;
        self.exec(
            SqlUpdater::new(ChnotTag::TABLE)
                .set(ChnotTag::OMIT_TID, OmitTID::now())
                .r#where(Wheres::and([
                    Wheres::equal(ChnotTag::META_OTID, meta_otid),
                    Wheres::equal(ChnotTag::OMIT_TID, OmitTID::never()),
                    if tags.is_empty() {
                        Wheres::None
                    } else {
                        Wheres::not(Wheres::r#in(ChnotTag::TAG, tags.clone()))
                    },
                ])),
        )
        .await?;
        let executor = KDbExecutor::Tx(self);
        if tags.is_empty() {
            tags.push(UNTAGGED_TAG.try_into()?);
        }

        for tag in tags {
            executor
                .exec(
                    ChnotTag {
                        tid: TID::default(),
                        kspace: kspace.to_owned(),
                        tag: tag.to_owned(),
                        meta_otid,
                        omit_tid: OmitTID::never(),
                    }
                    .to_sql_inserter()
                    .on_conflict(chin_sql::OnConflict::Ignore),
                )
                .await?;
        }

        Ok(())
    }

    pub(super) async fn chnot_overwrite(
        &self,
        req: KReq<ChnotOverwriteRecordReq>,
    ) -> AResult<ChnotOverwriteRecordRsp> {
        log::debug!("begin to overwrite chnot, {:?}", req.meta_otid);

        let meta_otid = match req.meta_otid {
            Some(tid) => MetaId::Old(tid),
            None => MetaId::New(TID::default()),
        };
        let archor: bool;

        let rec_tid = TID::default();

        let mut chnot_parser = ChnotParser::new(req.content.as_str());
        chnot_parser.parse();
        let todo_event = chnot_parser.get_outer_todo_event();

        let mut meta_tid = None;
        match meta_otid {
            MetaId::Old(meta_otid) => {
                // Query for existing record
                let query_old_rec = SqlBuilder::read(
                    ChnotRecord::TABLE,
                    &[ChnotRecord::META_OTID, ChnotRecord::CONTENT],
                )
                .r#where(Wheres::and([
                    Wheres::equal(ChnotRecord::META_OTID, meta_otid),
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
                let time_delta = rec_tid
                    .as_utc()
                    .signed_duration_since(old_id.as_utc())
                    .abs();
                archor = (textdistance::str::sift4_simple(&old_cont, req.content.as_str()) >= 60
                    && time_delta > TimeDelta::minutes(3))
                    || time_delta > TimeDelta::hours(1);

                let rec = ChnotRecord {
                    tid: rec_tid,
                    meta_otid,
                    omit_tid: OmitTID::never(),
                    content: req.content.clone(),
                    archor,
                    todo_event,
                };

                let update_omit = ChnotRecord::pkey_updater(meta_otid, OmitTID::never())
                    .set(ChnotRecord::OMIT_TID, OmitTID::now());
                self.exec(update_omit).await?;
                self.as_executor().chnot_record_insert(rec).await?;
            }
            MetaId::New(meta_otid) => {
                archor = true;
                let rec = ChnotRecord {
                    tid: rec_tid,
                    meta_otid,
                    omit_tid: OmitTID::never(),
                    content: req.content.clone(),
                    archor: true,
                    todo_event,
                };
                self.as_executor().chnot_record_insert(rec).await?;
                let tid = TID::default();
                meta_tid.replace(tid);
                let meta = ChnotMetadata {
                    otid: meta_otid,
                    kspace: req.kspace.clone(),
                    kind: req.kind.clone(),
                    pin_time: None,
                    omit_tid: OmitTID::never(),
                    archive_time: None,
                    tid,
                };
                self.as_executor().chnot_meta_insert(meta).await?;
            }
        }
        if let Some(kid) = req.kind_id.clone() {
            self.exec(
                ChnotKindRel::pkey_updater(*meta_otid, OmitTID::never())
                    .set(ChnotKindRel::OMIT_TID, OmitTID::now()),
            )
            .await?;
            self.exec(
                ChnotKindRel {
                    meta_otid: *meta_otid,
                    omit_tid: OmitTID::never(),
                    kind_id: kid.try_into()?,
                    tid: TID::default(),
                }
                .to_sql_inserter(),
            )
            .await?;
        }

        self.chnot_tag_update_single_chnot(
            ChnotTagUpdateReq {
                content: req.content.clone(),
                meta_otid: *meta_otid,
                kspace: req.kspace.clone(),
            },
            &chnot_parser,
        )
        .await?;

        Ok(ChnotOverwriteRecordRsp {
            meta_otid: *meta_otid,
            rec_tid,
            kspace: req.kspace,
            archor,
            todo_event,
            meta_tid,
        })
    }
}
