use super::mapper::ChnotMapper;
use super::*;
use crate::krate::llmchat::LLMChatRecord;
use crate::krate::mdwt::{MdwtRecord, MdwtRecordTable, MdwtTag};
use crate::mapper::Curd;
use crate::mapper::db::helper::{Ddls, create_tables};
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::KSerde;
use crate::model::dto::KReq;
use crate::util::string_util::StringUtils;
use chin_sql::time_type::TID;
use chin_sql::{ILikeType, LimitOffset, SqlBuilder};
use chin_sql::{Join, Wheres};
use chin_tools::{AResult, EResult};
use chrono::{FixedOffset, Local};

impl ChnotMapper for KDb {
    async fn ensure_table_chnot(&self) -> EResult {
        create_tables(
            Ddls::new()
                .with_ddls(ChnotThreadOrder::ddls())
                .with_ddls(ChnotThreadMeta::ddls())
                .with_ddls(ChnotMeta::ddls()),
            self,
        )
        .await
    }

    async fn chnot_thread_list(
        &self,
        req: KReq<ChnotThreadListReq>,
    ) -> AResult<ChnotThreadListRsp> {
        let ctm = ChnotThreadMetaTable::new("ctm");
        let cto = ChnotThreadOrderTable::new("cto");
        let cm = ChnotMetaTable::new("cm");
        let cont = MdwtRecordTable::new("cont");

        let mut sql_builder = SqlBuilder::new()
            .seg("select ctm.*, cm.otid as chnot_otid, cont.content as cont from")
            .merge(
                Join::first(&ctm)
                    .left_join(&cto, [(ctm.otid(), cto.thread_otid()).into()])
                    .left_join(&cm, [(cm.otid(), cto.otid()).into()]),
            );

        if let Some(query) = req.query.as_ref()
            && !query.is_empty()
        {
            let mdwt = SqlBuilder::read(
                MdwtRecord::TABLE,
                &[
                    &format!("{} content", MdwtRecord::CONTENT),
                    &format!("{} chnot_otid", MdwtRecord::OTID),
                ],
            )
            .r#where(Wheres::ilike(MdwtRecord::CONTENT, query, ILikeType::Fuzzy));

            let llmchat = SqlBuilder::read(
                LLMChatRecord::TABLE,
                &[
                    &format!("{} content", LLMChatRecord::CONTENT),
                    &format!("{} chnot_otid", LLMChatRecord::SESSION_OTID),
                ],
            )
            .r#where(Wheres::ilike(MdwtRecord::CONTENT, query, ILikeType::Fuzzy));

            let keyword_matcher = SqlBuilder::new().merge(mdwt).merge("union").merge(llmchat);

            sql_builder = sql_builder
                .seg("inner join (")
                .merge(keyword_matcher)
                .seg(") cont")
                .seg("on cont.chnot_otid = ")
                .seg(cm.otid().twn());
        } else {
            sql_builder = sql_builder
                .seg("left join")
                .seg(cont.nwa())
                .seg("on")
                .seg(cm.otid().twn())
                .seg("=")
                .seg(cont.otid().twn())
        }

        if let Some(tags) = req.tags.as_ref() {
            let mt = MdwtTag::mdwt_otids_sub(req.mkspaces.clone(), Some(tags));
            sql_builder = sql_builder
                .seg("inner join (")
                .merge(mt)
                .seg(") mt on")
                .seg("mt.otid = ")
                .seg(cm.otid().twn())
                .seg("or")
                .seg("mt.otid = ")
                .seg(cm.otid().twn())
        }

        sql_builder = sql_builder
            .r#where(Wheres::and([
                Wheres::transform(req.query.as_ref(), |q| {
                    if q.is_some() {
                        cto.korder().v_eq(0)
                    } else {
                        Wheres::None
                    }
                }),
                ctm.kspace().v_in(req.get_spaces()),
            ]))
            .seg("order by")
            .seg(ctm.pin_time().twn())
            .seg("desc,")
            .seg(ctm.otid().twn())
            .seg("desc,")
            .seg(cto.korder().twn())
            .seg("asc")
            .limit_offset(LimitOffset::new(req.page_size).offset(req.start_index));

        let data = self
            .conn()
            .await?
            .qry_list(sql_builder, |row| {
                Ok(ChnotThreadListRspData {
                    meta: ChnotThreadMeta {
                        otid: row.try_get(ChnotThreadMeta::OTID)?,
                        kspace: row.try_get(ChnotThreadMeta::KSPACE)?,
                        pin_time: row.try_get(ChnotThreadMeta::PIN_TIME)?,
                        archive_time: row.try_get(ChnotThreadMeta::ARCHIVE_TIME)?,
                        tid: row.try_get(ChnotThreadMeta::TID)?,
                    },
                    preview_text: row.try_get("cont")?,
                    chnot_otid: row.try_get("chnot_otid")?,
                })
            })
            .await?;

        Ok(ChnotThreadListRsp {
            next_start: req.start_index + data.len(),
            has_next: data.len() >= req.page_size,
            data,
        })
    }

    async fn chnot_thread_meta_commit(
        &self,
        req: KReq<ChnotThreadMetaFetchCommitReq>,
    ) -> AResult<ChnotThreadMetaFetchCommitRsp> {
        let mut conn = self.conn().await?;

        let tx = conn.tx().await?;

        let reader = ChnotThreadMeta::pkey_reader(req.meta_otid);
        let mut meta: ChnotThreadMeta =
            tx.qry_opt(reader, |e| (&e).try_into())
                .await?
                .unwrap_or(ChnotThreadMeta {
                    otid: req.meta_otid,
                    kspace: req.kspace.clone(),
                    pin_time: None,
                    archive_time: None,
                    tid: TID::default(),
                });

        tx.as_executor()
            .omit_rows::<ChnotThreadMeta>(ChnotThreadMeta::pkey_cond(req.meta_otid))
            .await?;

        meta.tid = TID::default();
        if let Some(pin_it) = req.pinned {
            if pin_it {
                meta.pin_time = Some(Local::now().fixed_offset());
            } else {
                meta.pin_time = None;
            }
        }

        if let Some(archive_it) = req.archive {
            if archive_it {
                meta.archive_time = Some(Local::now().fixed_offset());
            } else {
                meta.archive_time = None;
            }
        }

        if let Some(ksapce) = req.body.kspace {
            meta.kspace = ksapce;
        }

        tx.exec(meta.clone().to_sql_inserter()).await?;

        tx.cmt().await?;

        Ok(ChnotThreadMetaFetchCommitRsp { meta })
    }

    async fn chnot_overwrite_thread_orders(
        &self,
        req: KReq<ChnotThreadOrderCommitReq>,
    ) -> AResult<ChnotThreadOrderCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ChnotThreadOrderCommitReq {
            thread_otid,
            orders,
        } = req.body;

        let mut metas = vec![];
        for (c, b) in orders.iter().enumerate() {
            let rec = ChnotThreadOrder {
                otid: b.otid,
                tid: TID::default(),
                thread_otid,
                korder: c.try_into()?,
            };

            metas.push(rec.clone());

            tx.as_executor()
                .omit_rows::<ChnotThreadOrder>(rec.pkey())
                .await?;
            tx.exec(rec.to_sql_inserter()).await?;
        }

        tx.cmt().await?;
        Ok(ChnotThreadOrderCommitRsp {})
    }

    async fn chnot_thread_meta_fetch(
        &self,
        req: KReq<ChnotThreadMetaFetchReq>,
    ) -> AResult<ChnotThreadMetaFetchRsp> {
        let conn = self.conn().await?;
        let thread_meta = conn
            .qry_opt(ChnotThreadMeta::pkey_reader(req.thread_otid), |e| {
                ChnotThreadMeta::try_from(&e)
            })
            .await?;

        let metas = conn
            .qry_list(
                SqlBuilder::read(
                    ChnotMeta::TABLE,
                    &[format!("{}.*", ChnotMeta::TABLE).as_str()],
                )
                .seg(format!(
                    " left join {} on {}.{} = {}.{} ",
                    ChnotThreadOrder::TABLE,
                    ChnotMeta::TABLE,
                    ChnotMeta::OTID,
                    ChnotThreadOrder::TABLE,
                    ChnotThreadOrder::OTID
                ))
                .r#where(Wheres::and([Wheres::equal(
                    ChnotThreadOrder::THREAD_OTID.prefix_with_sep(ChnotThreadOrder::TABLE, "."),
                    req.thread_otid,
                )])),
                |row| ChnotMeta::try_from(&row),
            )
            .await?;
        /*         let toents = conn
        .qry_list(
            SqlBuilder::read_all(MdwtToent::TABLE).r#where(Wheres::equal(
                MdwtToent::CHNOT_OTID,
                req.chnot_meta_otid,
            )),
            |row| MdwtToent::try_from(&row),
        )
        .await?; */

        Ok(ChnotThreadMetaFetchRsp {
            thread_meta,
            chnot_meta_sorted: metas,
        })
    }

    async fn chnot_meta_commit(
        &self,
        req: KReq<ChnotMetaCommitReq>,
    ) -> AResult<ChnotMetaCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ChnotMetaCommitReq { metas } = req.body;

        let mut result_metas = vec![];
        for b in metas {
            let rec = ChnotMeta {
                otid: b.otid,
                tid: TID::default(),
                kind: b.kind,
                kspace: b.kspace,
                archive_time: if b.archive.is_some_and(|v| v) {
                    Some(Local::now().fixed_offset())
                } else {
                    None
                },
            };

            result_metas.push(rec.clone());

            tx.as_executor().omit_rows::<ChnotMeta>(rec.pkey()).await?;
            tx.exec(rec.to_sql_inserter()).await?;
        }

        tx.cmt().await?;
        Ok(ChnotMetaCommitRsp {
            metas: result_metas,
        })
    }

    async fn chnot_meta_list(&self, req: KReq<ChnotMetaListReq>) -> AResult<ChnotMetaListRsp> {
        if req.body.otids.is_empty() {
            return Ok(ChnotMetaListRsp { metas: vec![] });
        }
        let cm = ChnotMetaTable::new("cm");
        let metas = SqlBuilder::read_all(&cm.nwa()).r#where(cm.otid().v_in(req.body.otids));
        let cms = self
            .conn()
            .await?
            .qry_list(metas, |row| ChnotMeta::try_from_kdb_row(&row))
            .await?;
        Ok(ChnotMetaListRsp { metas: cms })
    }
}

impl TryFrom<&KDbRow> for ChnotThreadMeta {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = ChnotThreadMeta {
            otid: value.try_get(ChnotThreadMeta::OTID)?,
            kspace: value.try_get(ChnotThreadMeta::KSPACE)?,
            pin_time: value.try_get(ChnotThreadMeta::PIN_TIME)?,
            archive_time: value.try_get(ChnotThreadMeta::ARCHIVE_TIME)?,
            tid: value.try_get(ChnotThreadMeta::TID)?,
        };
        Ok(chnot)
    }
}
