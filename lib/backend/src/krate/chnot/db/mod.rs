pub(crate) mod creater;

use super::mapper::ChnotMapper;
use super::*;
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
use anyhow::anyhow;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{ILikeType, SegOrVal, SqlBuilder};
use chin_sql::{LimitOffset, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;
use itertools::Itertools;
use log::info;
use serde::Serialize;

const UNTAGGED_TAG: &str = "<NON>";

#[inline]
fn chnot_query_sql<'a>() -> SqlBuilder<'a> {
    SqlBuilder::new()
    .sov("SELECT r.tid as rec_tid, r.content, r.archor,")
    .sov("tm.otid as meta_otid, tm.kspace, tm.pin_time, tm.archive_time, r.todo_event, tm.tid as meta_tid")
    .sov("FROM chnot_thread_meta tm left join chnot_meta cm on tm.otid = cm.thread_otid and cm.korder=0 and cm.kind = 'mdwt' left join mdwt_record r ON cast(cm.kind_id as bigint) = r.otid")
}

impl ChnotThreadTag {
    fn with_those_tag_meta_otids<'a>(
        kspaces: Vec<Varchar<40>>,
        tags: Option<&ChnotThreadTagSearchType>,
    ) -> SqlBuilder<'a> {
        let v = vec![];
        let tags = match tags {
            Some(tags) => match tags {
                ChnotThreadTagSearchType::Inset(items) => items,
            },
            None => &v,
        };
        let len = if !tags.is_empty() {
            Some(tags.len())
        } else {
            None
        };
        SqlBuilder::read(ChnotThreadTag::TABLE, &[ChnotThreadTag::THREAD_OTID])
            .r#where(Wheres::and([
                Wheres::r#in(ChnotThreadTag::KSPACE, kspaces),
                Wheres::if_some(len, |_| Wheres::r#in(ChnotThreadTag::TAG, tags.to_vec())),
            ]))
            .sov("group by")
            .sov(ChnotThreadTag::THREAD_OTID)
            .some_then(len, |l, sb| {
                sb.sov("having")
                    .sov(format!("COUNT(DISTINCT {}) = ", ChnotThreadTag::TAG))
                    .sov(SegOrVal::val(l as i64))
            })
    }
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

impl KDb {
    async fn chnot_tag_query_inner<F, T>(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
        mapper: F,
        name_only: bool,
    ) -> AResult<ChnotThreadTagQueryRsp<T>>
    where
        F: Fn(KDbRow) -> AResult<T> + Send + 'static,
        T: Serialize + Clone + Send + 'static + AsRef<str>,
    {
        let field = if name_only { "distinct tag" } else { "*" };

        let sql = SqlBuilder::new()
            .sov("WITH qualified_tids AS (")
            .merge(ChnotThreadTag::with_those_tag_meta_otids(
                req.get_spaces(),
                req.tags.as_ref(),
            ))
            .sov(")")
            .merge(
                SqlBuilder::read(ChnotThreadTag::TABLE, &[field])
                    .sov("as t")
                    .sov("right join qualified_tids q on t.meta_otid = q.meta_otid")
                    .r#where(Wheres::and([Wheres::r#in(
                        ChnotThreadTag::KSPACE,
                        req.get_spaces(),
                    )]))
                    .limit_offset(LimitOffset::new(req.page_size).offset(req.start_index)),
            );
        let data = self.conn().await?.qry_list(sql, mapper).await?;

        Ok(ChnotThreadTagQueryRsp {
            data,
            start_index: req.start_index,
        })
    }
}

impl ChnotMapper for KDb {
    async fn ensure_table_chnot(&self) -> EResult {
        create_tables(
            vec![
                ChnotThreadTag::create_sql().to_owned_sql(),
                ChnotThreadTag::hist_table(),
                ChnotThreadMeta::create_sql().to_owned_sql(),
                ChnotThreadMeta::hist_table(),
                MdwtRecord::create_sql().to_owned_sql(),
                MdwtRecord::hist_table(),
                ChnotMeta::create_sql().to_owned_sql(),
                ChnotMeta::hist_table(),
                ChnotToent::create_sql().to_owned_sql(),
                ChnotToent::hist_table(),
            ],
            self,
        )
        .await
    }

    async fn chnot_thread_query(
        &self,
        req: KReq<ChnotThreadQueryReq>,
    ) -> AResult<ChnotThreadQueryRsp> {
        let page_size = req.page_size;
        let page_start = req.start_index;

        let chnot_sql = SqlBuilder::new()
            .sov("select * from ")
            .sub("t", chnot_query_sql())
            .some_then(req.tags.as_ref(), |tag, sr| {
                sr.sov("inner join")
                    .sub(
                        "ct",
                        ChnotThreadTag::with_those_tag_meta_otids(req.get_spaces(), Some(tag)),
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

        Ok(ChnotThreadQueryRsp {
            has_next: cs.len() >= page_size,
            data: cs,
            next_start: page_start + page_size,
        })
    }

    async fn chnot_overwrite_thread_meta(
        &self,
        req: KReq<ChnotOverwriteThreadMetaReq>,
    ) -> AResult<ChnotOverwriteThreadMetaRsp> {
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

        Ok(ChnotOverwriteThreadMetaRsp { meta })
    }

    async fn chnot_overwrite_mdwt(
        &self,
        req: KReq<ChnotOverwriteMdwtReq>,
    ) -> AResult<ChnotOverwriteMdwtRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let rsp = tx.chnot_overwrite_mdwt(req).await;

        if rsp.is_ok() {
            tx.cmt().await?;
        } else {
            tx.rbk().await?;
        }

        rsp
    }

    async fn chnot_overwrite_metas(
        &self,
        req: KReq<ChnotOverwriteMetaReq>,
    ) -> AResult<ChnotOverwriteMetaRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ChnotOverwriteMetaReq {
            thread_otid: meta_otid,
            metas,
        } = req.body;
        let rsp = tx.overwrite_block_metas(metas, meta_otid).await;

        if let Ok(rsp) = rsp {
            tx.cmt().await?;
            Ok(ChnotOverwriteMetaRsp { metas: rsp })
        } else {
            tx.rbk().await?;
            Err(anyhow!("unable to insert chnot metas"))
        }
    }

    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
    ) -> AResult<ChnotThreadTagQueryRsp<ChnotThreadTag>> {
        self.chnot_tag_query_inner(req, |e| (&e).try_into(), false)
            .await
    }

    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotThreadTagQueryReq>,
    ) -> AResult<ChnotThreadTagQueryRsp<String>> {
        info!("{req:#?}");
        let remove_params = req.remove_params.default_true();
        let input_tag = req.body.tags.as_ref().map_or(vec![], |c| match c {
            ChnotThreadTagSearchType::Inset(items) => items.to_vec(),
        });
        let mut result: ChnotThreadTagQueryRsp<String> = self
            .chnot_tag_query_inner(req, |e| e.try_get(ChnotThreadTag::TAG), true)
            .await?;

        result.data = result
            .data
            .into_iter()
            .unique()
            .filter(|e| {
                if remove_params {
                    !input_tag.contains(e)
                } else {
                    true
                }
            })
            .collect();

        Ok(result)
    }

    async fn chnot_tag_update_all(&self, kspace: Varchar<40>) -> EResult {
        let get_all = SqlBuilder::read(MdwtRecord::TABLE, &[MdwtRecord::CONTENT, MdwtRecord::OTID])
            .r#where(Wheres::and([Wheres::compare_str(
                MdwtRecord::OTID,
                "in",
                format!(
                    "(select {} from {} where kspace = '{}')",
                    ChnotThreadMeta::OTID,
                    ChnotThreadMeta::TABLE,
                    kspace.as_str().replace("'", "<quote>")
                ),
            )]));

        let kspace = kspace.to_owned();
        let mut conn = self.conn().await?;
        let chnots = conn
            .qry_list(get_all, move |e| {
                Ok(ChnotThreadTagUpdateReq {
                    content: e.try_get(MdwtRecord::CONTENT)?,
                    thread_otid: e.try_get(MdwtRecord::OTID)?,
                    kspace: kspace.to_owned(),
                })
            })
            .await?;

        let tx = conn.transaction().await?;
        for one in chnots {
            let chnot_parser = parser::ChnotParser::new(one.content.as_str());
            tx.chnot_tag_update_single_chnot(one.clone(), &chnot_parser)
                .await?;
        }
        tx.cmt().await?;

        Ok(())
    }

    async fn chnot_thread_meta(
        &self,
        req: KReq<ChnotThreadMetaReq>,
    ) -> AResult<ChnotThreadMetaRsp> {
        let conn = self.conn().await?;
        let chnot_meta = conn
            .qry_one(
                ChnotThreadMeta::pkey_reader(req.thread_otid),
                |e| ChnotThreadMeta::try_from(&e),
                false,
            )
            .await?;

        let block_metas = conn
            .qry_list(
                SqlBuilder::read_all(ChnotMeta::TABLE)
                    .r#where(Wheres::equal(ChnotMeta::THREAD_OTID, req.thread_otid)),
                |row| ChnotMeta::try_from(&row),
            )
            .await?;
        /*         let toents = conn
        .qry_list(
            SqlBuilder::read_all(ChnotToent::TABLE).r#where(Wheres::equal(
                ChnotToent::CHNOT_OTID,
                req.chnot_meta_otid,
            )),
            |row| ChnotToent::try_from(&row),
        )
        .await?; */

        Ok(ChnotThreadMetaRsp {
            thread_meta: chnot_meta,
            chnot_meta_sorted: block_metas,
        })
    }

    async fn mdwt_blocks(&self, req: KReq<MdwtRecordsReq>) -> AResult<MdwtRecordsRsp> {
        let recs = self
            .conn()
            .await?
            .qry_list(
                SqlBuilder::read_all(MdwtRecord::TABLE).r#where(Wheres::or([Wheres::r#in(
                    MdwtRecord::OTID,
                    req.body.mdwt_otids,
                )])),
                |row| MdwtRecord::try_from(&row),
            )
            .await?;

        Ok(MdwtRecordsRsp {
            mdwt_map: recs.into_iter().map(|r| (r.otid, r)).collect(),
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

impl TryFrom<&KDbRow> for MdwtRecord {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = Self {
            content: value.try_get(Self::CONTENT)?,
            tid: value.try_get(Self::TID)?,
            archor: value.try_get(Self::ARCHOR)?,
            todo_event: {
                let opt: Option<String> = value.try_get(Self::TODO_EVENT)?;
                match opt {
                    Some(opt) => Some(TodoEvent::try_from_standrd_str(opt.as_str())?),
                    None => None,
                }
            },
            otid: value.try_get(Self::OTID)?,
        };
        Ok(chnot)
    }
}

impl TryFrom<&KDbRow> for ChnotThreadTag {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = ChnotThreadTag {
            tid: value.try_get(ChnotThreadTag::TID)?,
            kspace: value.try_get(ChnotThreadTag::KSPACE)?,
            tag: value.try_get(ChnotThreadTag::TAG)?,
            thread_otid: value.try_get(ChnotThreadTag::THREAD_OTID)?,
        };
        Ok(obj)
    }
}
