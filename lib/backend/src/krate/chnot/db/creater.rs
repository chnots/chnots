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
        self.as_executor()
            .omit_rows::<ChnotTag>(Wheres::and([
                Wheres::equal(ChnotTag::META_OTID, meta_otid),
                if tags.is_empty() {
                    Wheres::None
                } else {
                    Wheres::not(Wheres::r#in(ChnotTag::TAG, tags.clone()))
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
                    ChnotTag {
                        tid: TID::default(),
                        kspace: kspace.to_owned(),
                        tag: tag.to_owned(),
                        meta_otid,
                    }
                    .to_sql_inserter()
                    .on_conflict(chin_sql::OnConflict::Ignore),
                )
                .await?;
        }

        Ok(())
    }

    async fn overwrite_block_metas(
        &self,
        blocks: Vec<ChnotOverwriteRecordReqMeta>,
        chnot_meta_otid: TID,
    ) -> EResult {
        for b in blocks {
            let rec = ChnotBlockMeta {
                otid: b.block_otid,
                chnot_otid: chnot_meta_otid,
                kind: b.kind,
                kind_id: b.kind_id,
                korder: b.korder,
                tid: TID::default(),
            };

            self.as_executor()
                .omit_rows::<ChnotBlockMeta>(rec.pkey())
                .await?;
            self.exec(rec.to_sql_inserter()).await?;
        }
        Ok(())
    }

    async fn overwrite_block_record(
        &self,
        block: ChnotOverwriteRecordReqRecord,
        chnot_meta_otid: TID,
    ) -> EResult {
        struct OldInfo {
            tid: TID,
            content: String,
        }
        fn to_old_info(row: KDbRow) -> AResult<OldInfo> {
            Ok(OldInfo {
                tid: row.try_get(ChnotBlockRecord::BLOCK_OTID)?,
                content: row.try_get(ChnotBlockRecord::CONTENT)?,
            })
        }

        // TODO: parse content and backlinks
        let chnot_parser = ChnotParser::new(block.content.as_str());
        let rec_tid: TID = TID::default();

        // Query for existing record
        let query_old_rec = SqlBuilder::read(
            ChnotBlockRecord::TABLE,
            &[ChnotBlockRecord::BLOCK_OTID, ChnotBlockRecord::CONTENT],
        )
        .r#where(Wheres::and([Wheres::equal(
            ChnotBlockRecord::BLOCK_OTID,
            block.block_otid,
        )]));

        // Get old record info
        let Some(OldInfo {
            tid: old_id,
            content: old_cont,
        }) = self.qry_opt(query_old_rec, to_old_info).await?
        else {
            return Err(anyhow::anyhow!("Old Chnot is absent"));
        };

        let time_delta = rec_tid
            .as_utc()
            .signed_duration_since(old_id.as_utc())
            .abs();

        if old_cont.len() == block.content.as_str().len() && old_cont == block.content.as_str() {
            return Ok(());
        }

        let archor: bool = (textdistance::str::sift4_simple(&old_cont, block.content.as_str())
            >= 60
            && time_delta > TimeDelta::minutes(3))
            || time_delta > TimeDelta::hours(1);

        let rec = ChnotBlockRecord {
            block_otid: block.block_otid,
            meta_otid: chnot_meta_otid,
            tid: rec_tid,
            todo_event: None,
            content: block.content,
            archor,
        };

        self.as_executor()
            .omit_rows::<ChnotBlockRecord>(ChnotBlockRecord::pkey_cond(block.block_otid))
            .await?;
        self.exec(rec.to_sql_inserter()).await?;

        Ok(())
    }

    pub(super) async fn chnot_overwrite_records(
        &self,
        req: KReq<ChnotOverwriteRecordReq>,
    ) -> AResult<ChnotOverwriteRecordRsp> {
        let ChnotOverwriteRecordReq {
            meta_otid,
            recs,
            metas,
        } = req.body;

        for block in recs {
            self.overwrite_block_record(block, meta_otid).await?;
        }

        self.overwrite_block_metas(metas, meta_otid).await?;

        Ok(ChnotOverwriteRecordRsp { todo_event: None })
    }
}
