use super::*;
use crate::krate::chnot::parser::ChnotParser;
use crate::mapper::db::{KDbExecutorBehaiver, KDbRow, KDbTx};
use crate::model::dto::KReq;
use chin_sql::SqlBuilder;
use chin_sql::time_type::TID;
use chin_sql::{ChinSqlError, Wheres};
use chin_tools::AResult;

use chrono::TimeDelta;

impl<'a> KDbTx<'a> {
    pub(super) async fn chnot_tag_update_single_chnot(
        &self,
        req: ChnotThreadTagUpdateReq,
        chnot_parser: &ChnotParser<'_>,
    ) -> EResult {
        let ChnotThreadTagUpdateReq {
            content: _,
            thread_otid: meta_otid,
            kspace,
        } = req;

        let tags: Result<Vec<Varchar<800>>, ChinSqlError> = chnot_parser
            .get_all_tags()
            .iter()
            .map(|t| t.to_string().try_into())
            .collect();
        let mut tags = tags?;
        self.as_executor()
            .omit_rows::<ChnotThreadTag>(Wheres::and([
                Wheres::equal(ChnotThreadTag::THREAD_OTID, meta_otid),
                if tags.is_empty() {
                    Wheres::None
                } else {
                    Wheres::not(Wheres::r#in(ChnotThreadTag::TAG, tags.clone()))
                },
            ]))
            .await?;
        let executor = self.as_executor();
        if tags.is_empty() {
            tags.push(UNTAGGED_TAG.try_into()?);
        }

        for tag in tags {
            executor
                .exec(
                    ChnotThreadTag {
                        tid: TID::default(),
                        kspace: kspace.to_owned(),
                        tag: tag.to_owned(),
                        thread_otid: meta_otid,
                    }
                    .to_sql_inserter()
                    .on_conflict(chin_sql::OnConflict::Ignore),
                )
                .await?;
        }

        Ok(())
    }

    pub(super) async fn overwrite_block_metas(
        &self,
        blocks: Vec<ChnotOverwriteMetaReqData>,
        chnot_meta_otid: TID,
    ) -> EResult {
        for b in blocks {
            let rec = ChnotMeta {
                otid: b.otid,
                thread_otid: chnot_meta_otid,
                kind: b.kind,
                kind_id: b.kind_id,
                korder: b.korder,
                tid: TID::default(),
            };

            self.as_executor()
                .omit_rows::<ChnotMeta>(rec.pkey())
                .await?;
            self.exec(rec.to_sql_inserter()).await?;
        }
        Ok(())
    }

    async fn overwrite_block_record(
        &self,
        block: ChnotOverwriteMdwtReqData,
        chnot_meta_otid: TID,
    ) -> EResult {
        struct OldInfo {
            tid: TID,
            content: String,
        }
        fn to_old_info(row: KDbRow) -> AResult<OldInfo> {
            Ok(OldInfo {
                tid: row.try_get(MdwtRecord::OTID)?,
                content: row.try_get(MdwtRecord::CONTENT)?,
            })
        }

        // TODO: parse content and backlinks
        let chnot_parser = ChnotParser::new(block.content.as_str());
        let rec_tid: TID = TID::default();

        // Query for existing record
        let query_old_rec =
            SqlBuilder::read(MdwtRecord::TABLE, &[MdwtRecord::OTID, MdwtRecord::CONTENT])
                .r#where(Wheres::and([Wheres::equal(MdwtRecord::OTID, block.otid)]));

        // Get old record info
        let archor = if let Some(OldInfo {
            tid: old_id,
            content: old_cont,
        }) = self.qry_opt(query_old_rec, to_old_info).await?
        {
            let time_delta = rec_tid
                .as_utc()
                .signed_duration_since(old_id.as_utc())
                .abs();

            if old_cont.len() == block.content.as_str().len() && old_cont == block.content.as_str()
            {
                return Ok(());
            }

            (textdistance::str::sift4_simple(&old_cont, block.content.as_str()) >= 60
                && time_delta > TimeDelta::minutes(3))
                || time_delta > TimeDelta::hours(1)
        } else {
            false
        };

        let rec = MdwtRecord {
            otid: block.otid,
            tid: rec_tid,
            todo_event: None,
            content: block.content,
            archor,
        };

        self.as_executor()
            .omit_rows::<MdwtRecord>(MdwtRecord::pkey_cond(block.otid))
            .await?;
        self.exec(rec.to_sql_inserter()).await?;

        Ok(())
    }

    pub(super) async fn chnot_overwrite_mdwts(
        &self,
        req: KReq<ChnotOverwriteMdwtReq>,
    ) -> AResult<ChnotOverwriteMdwtRsp> {
        let ChnotOverwriteMdwtReq {
            thread_otid: meta_otid,
            mdwts: recs,
        } = req.body;

        for block in recs {
            self.overwrite_block_record(block, meta_otid).await?;
        }

        Ok(ChnotOverwriteMdwtRsp { todo_event: None })
    }
}
