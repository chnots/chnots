use super::mapper::ChnotMapper;
use super::*;
use crate::krate::mdwt::{MdwtRecord, MdwtTag};
use crate::krate::toent::logic::EventBuilder;
use crate::krate::toent::logic::todoevent::TodoEvent;
use crate::mapper::Curd;
use crate::mapper::db::helper::create_tables;
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::util::result_util::UnwrapOr;
use chin_sql::time_type::TID;
use chin_sql::{ILikeType, SqlBuilder};
use chin_sql::{LimitOffset, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;
use itertools::Itertools;

const UNTAGGED_TAG: &str = "<NON>";

#[inline]
fn chnot_query_sql<'a>() -> SqlBuilder<'a> {
    SqlBuilder::new()
    .sov("SELECT r.tid as rec_tid, r.content, r.archor,")
    .sov("tm.otid as meta_otid, tm.kspace, tm.pin_time, tm.archive_time, r.todo_event, tm.tid as meta_tid")
    .sov("FROM chnot_thread_meta tm left join chnot_meta cm on tm.otid = cm.thread_otid and cm.korder=0 and cm.kind = 'mdwt' left join mdwt_record r ON cast(cm.kind_id as bigint) = r.otid")
}

#[inline]
fn chnot_thread_query_mapper(row: KDbRow) -> AResult<ChnotThread> {
    let chnot = if let (Ok(tid), Ok(content), Ok(archor), Ok(todo_event), Ok(otid)) = (
        row.try_get("rec_tid"),
        row.try_get("content"),
        row.try_get("archor"),
        {
            let opt: AResult<Option<String>> = row.try_get("todo_event");
            match opt {
                Ok(Some(opt)) => {
                    let c: AResult<Option<TodoEvent>> =
                        Ok(Some(TodoEvent::try_from_standrd_str(opt.as_str())?));
                    c
                }

                Ok(None) => Ok(None),
                Err(_) => Ok(None),
            }
        },
        row.try_get("meta_otid"),
    ) {
        Some(MdwtRecord {
            tid,
            content,
            archor,
            todo_event,
            otid,
        })
    } else {
        None
    };

    let meta = ChnotThreadMeta {
        otid: row.try_get("meta_otid")?,
        kspace: row.try_get("kspace")?,
        pin_time: row.try_get("pin_time")?,
        archive_time: row.try_get("archive_time")?,
        tid: row.try_get("meta_tid")?,
    };
    Ok(ChnotThread {
        head_content: chnot.as_ref().map(|c| c.content.clone()),
        todo_event: chnot.and_then(|c| c.todo_event),
        meta,
    })
}

impl ChnotMapper for KDb {
    async fn ensure_table_chnot(&self) -> EResult {
        create_tables(
            vec![
                ChnotThreadMeta::create_sql().to_owned_sql(),
                ChnotThreadMeta::hist_table(),
                ChnotMeta::create_sql().to_owned_sql(),
                ChnotMeta::hist_table(),
            ],
            self,
        )
        .await
    }

    async fn chnot_thread_list(
        &self,
        req: KReq<ChnotThreadListReq>,
    ) -> AResult<ChnotThreadListRsp> {
        let page_size = req.page_size;
        let page_start = req.start_index;

        let chnot_sql = SqlBuilder::new()
            .sov("select * from ")
            .sub("t", chnot_query_sql())
            .some_then(req.tags.as_ref(), |tag, sr| {
                sr.sov("inner join")
                    .sub(
                        "ct",
                        MdwtTag::with_those_tag_meta_otids(req.get_spaces(), Some(tag)),
                    )
                    .sov("on t.meta_otid = ct.meta_otid")
            })
            .r#where(Wheres::and([
                // TODO: group by perm tid
                Wheres::transform(req.with_archive, |e| {
                    if e.default_false() {
                        Wheres::None
                    } else {
                        Wheres::is_null("archive_time")
                    }
                }),
                Wheres::r#in(
                    "t.kspace",
                    req.mkspaces
                        .clone()
                        .into_iter()
                        .merge(vec![req.kspace.clone()])
                        .collect(),
                ),
                Wheres::if_some(req.query.as_ref(), |content| {
                    Wheres::ilike("t.content", content, ILikeType::Fuzzy)
                }),
                Wheres::if_some(req.thread_otid, |tid| Wheres::equal("t.meta_otid", tid)),
            ]))
            .sov("ORDER BY t.pin_time asc, t.meta_otid desc")
            .custom(LimitOffset::new(req.page_size).offset_if_some(Some(req.start_index)));

        let cs = self
            .conn()
            .await?
            .qry_list(chnot_sql, chnot_thread_query_mapper)
            .await?;

        Ok(ChnotThreadListRsp {
            has_next: cs.len() >= page_size,
            data: cs,
            next_start: page_start + page_size,
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
        req: KReq<chnotThreadOrderCommitReq>,
    ) -> AResult<chnotThreadOrderCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let chnotThreadOrderCommitReq {
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

            tx.as_executor().omit_rows::<ChnotMeta>(rec.pkey()).await?;
            tx.exec(rec.to_sql_inserter()).await?;
        }

        tx.cmt().await?;
        Ok(chnotThreadOrderCommitRsp {})
    }

    async fn chnot_thread_meta_fetch(
        &self,
        req: KReq<ChnotThreadMetaFetchReq>,
    ) -> AResult<ChnotThreadMetaFetchRsp> {
        let conn = self.conn().await?;
        let chnot_meta = conn
            .qry_one(
                ChnotThreadMeta::pkey_reader(req.thread_otid),
                |e| ChnotThreadMeta::try_from(&e),
                false,
            )
            .await?;

        let thread = conn
            .qry_list(
                SqlBuilder::read(
                    ChnotMeta::TABLE,
                    &[format!("{}.*", ChnotMeta::TABLE).as_str()],
                )
                .sov(format!(
                    " left join {} on {}.{} = {}.{} ",
                    ChnotThreadOrder::TABLE,
                    ChnotMeta::OTID,
                    ChnotMeta::OTID,
                    ChnotThreadOrder::TABLE,
                    ChnotThreadOrder::OTID
                ))
                .r#where(Wheres::and([Wheres::equal(
                    ChnotThreadOrder::THREAD_OTID,
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
            thread_meta: chnot_meta,
            chnot_meta_sorted: thread,
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
                kind_id: b.kind_id,
                kspace: b.kspace,
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
