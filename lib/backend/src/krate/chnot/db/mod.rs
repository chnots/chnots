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
    .sov("m.otid as meta_otid, m.kspace, m.kind, m.pin_time, m.archive_time, r.todo_event, m.tid as meta_tid")
    .sov("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_otid = m.otid and r.meta_otid = r.block_otid")
}

impl ChnotTag {
    fn with_those_tag_meta_otids<'a>(
        kspaces: Vec<Varchar<40>>,
        tags: Option<&ChnotTagSearchType>,
    ) -> SqlBuilder<'a> {
        let v = vec![];
        let tags = match tags {
            Some(tags) => match tags {
                ChnotTagSearchType::Inset(items) => items,
            },
            None => &v,
        };
        let len = if !tags.is_empty() {
            Some(tags.len())
        } else {
            None
        };
        SqlBuilder::read(ChnotTag::TABLE, &[ChnotTag::META_OTID])
            .r#where(Wheres::and([
                Wheres::r#in(ChnotTag::KSPACE, kspaces),
                Wheres::if_some(len, |_| Wheres::r#in(ChnotTag::TAG, tags.to_vec())),
            ]))
            .sov("group by")
            .sov(ChnotTag::META_OTID)
            .some_then(len, |l, sb| {
                sb.sov("having")
                    .sov(format!("COUNT(DISTINCT {}) = ", ChnotTag::TAG))
                    .sov(SegOrVal::val(l as i64))
            })
    }
}

#[inline]
fn chnot_query_mapper(row: KDbRow) -> AResult<Chnot> {
    let record = MdwtRecord {
        tid: row.try_get("rec_tid")?,
        content: row.try_get("content")?,
        archor: row.try_get("archor")?,
        todo_event: {
            let opt: Option<String> = row.try_get("todo_event")?;
            match opt {
                Some(opt) => Some(TodoEvent::try_from_standrd_str(opt.as_str())?),
                None => None,
            }
        },
        otid: row.try_get(MdwtRecord::OTID)?,
    };
    let meta = ChnotMetadata {
        otid: row.try_get("meta_otid")?,
        kspace: row.try_get("kspace")?,
        pin_time: row.try_get("pin_time")?,
        archive_time: row.try_get("archive_time")?,
        tid: row.try_get("meta_tid")?,
    };
    Ok(Chnot {
        head_record: record,
        meta,
    })
}

impl KDb {
    async fn chnot_tag_query_inner<F, T>(
        &self,
        req: KReq<ChnotTagQueryReq>,
        mapper: F,
        name_only: bool,
    ) -> AResult<ChnotTagQueryRsp<T>>
    where
        F: Fn(KDbRow) -> AResult<T> + Send + 'static,
        T: Serialize + Clone + Send + 'static + AsRef<str>,
    {
        let field = if name_only { "distinct tag" } else { "*" };

        let sql = SqlBuilder::new()
            .sov("WITH qualified_tids AS (")
            .merge(ChnotTag::with_those_tag_meta_otids(
                req.get_spaces(),
                req.tags.as_ref(),
            ))
            .sov(")")
            .merge(
                SqlBuilder::read(ChnotTag::TABLE, &[field])
                    .sov("as t")
                    .sov("right join qualified_tids q on t.meta_otid = q.meta_otid")
                    .r#where(Wheres::and([Wheres::r#in(
                        ChnotTag::KSPACE,
                        req.get_spaces(),
                    )]))
                    .limit_offset(LimitOffset::new(req.page_size).offset(req.start_index)),
            );
        let data = self.conn().await?.qry_list(sql, mapper).await?;

        Ok(ChnotTagQueryRsp {
            data,
            start_index: req.start_index,
        })
    }
}

impl ChnotMapper for KDb {
    async fn ensure_table_chnot(&self) -> EResult {
        create_tables(
            vec![
                ChnotTag::create_sql().to_owned_sql(),
                ChnotTag::hist_table(),
                ChnotMetadata::create_sql().to_owned_sql(),
                ChnotMetadata::hist_table(),
                MdwtRecord::create_sql().to_owned_sql(),
                MdwtRecord::hist_table(),
                ChnotBlockMeta::create_sql().to_owned_sql(),
                ChnotBlockMeta::hist_table(),
                ChnotBlockToent::create_sql().to_owned_sql(),
                ChnotBlockToent::hist_table(),
            ],
            self,
        )
        .await
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Chnot>> {
        let page_size = req.page_size;
        let page_start = req.start_index;

        let chnot_sql = SqlBuilder::new()
            .sov("select * from ")
            .sub("t", chnot_query_sql())
            .some_then(req.tags.as_ref(), |tag, sr| {
                sr.sov("inner join")
                    .sub(
                        "ct",
                        ChnotTag::with_those_tag_meta_otids(req.get_spaces(), Some(tag)),
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
                Wheres::transform(&req.kinds, |k| {
                    if !k.is_empty() {
                        Wheres::r#in("t.kind", k.iter().map(|e| e.as_static_str()).collect())
                    } else {
                        Wheres::None
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
                Wheres::if_some(req.record_otid, |tid| Wheres::equal("t.rec_tid", tid)),
                Wheres::if_some(req.meta_otid, |tid| Wheres::equal("t.meta_otid", tid)),
            ]))
            .sov("ORDER BY t.pin_time asc, t.meta_otid desc")
            .custom(LimitOffset::new(req.page_size).offset_if_some(Some(req.start_index)));

        let cs = self
            .conn()
            .await?
            .qry_list(chnot_sql, chnot_query_mapper)
            .await?;

        Ok(ChnotQueryRsp {
            has_next: cs.len() >= page_size,
            data: cs,
            next_start: page_start + page_size,
        })
    }

    async fn chnot_overwrite_meta(
        &self,
        req: KReq<ChnotOverwriteMetaReq>,
    ) -> AResult<ChnotOverwriteMetaRsp> {
        let mut conn = self.conn().await?;

        let tx = conn.tx().await?;

        let reader = ChnotMetadata::pkey_reader(req.meta_otid);
        let mut meta: ChnotMetadata = tx.qry_one(reader, |e| (&e).try_into(), false).await?;

        tx.as_executor()
            .omit_rows::<ChnotMetadata>(ChnotMetadata::pkey_cond(req.meta_otid))
            .await?;

        meta.tid = TID::default();
        if let Some(o) = req.pinned {
            if o {
                meta.pin_time = Some(Local::now().fixed_offset());
            } else {
                meta.pin_time = None;
            }
        }

        if let Some(o) = req.archive {
            if o {
                meta.archive_time = Some(Local::now().fixed_offset());
            } else {
                meta.archive_time = None;
            }
        }

        if let Some(ksapce) = req.body.kspace {
            meta.kspace = ksapce;
        }

        tx.exec(meta.to_sql_inserter()).await?;

        tx.cmt().await?;

        Ok(ChnotOverwriteMetaRsp {})
    }

    async fn chnot_overwrite_records(
        &self,
        req: KReq<ChnotOverwriteBlockReq>,
    ) -> AResult<ChnotOverwriteRecordRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let rsp = tx.chnot_overwrite_records(req).await;

        if rsp.is_ok() {
            tx.cmt().await?;
        } else {
            tx.rbk().await?;
        }

        rsp
    }

    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>> {
        self.chnot_tag_query_inner(req, |e| (&e).try_into(), false)
            .await
    }

    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>> {
        info!("{req:#?}");
        let remove_params = req.remove_params.default_true();
        let input_tag = req.body.tags.as_ref().map_or(vec![], |c| match c {
            ChnotTagSearchType::Inset(items) => items.to_vec(),
        });
        let mut result: ChnotTagQueryRsp<String> = self
            .chnot_tag_query_inner(req, |e| e.try_get(ChnotTag::TAG), true)
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
                    ChnotMetadata::OTID,
                    ChnotMetadata::TABLE,
                    kspace.as_str().replace("'", "<quote>")
                ),
            )]));

        let kspace = kspace.to_owned();
        let mut conn = self.conn().await?;
        let chnots = conn
            .qry_list(get_all, move |e| {
                Ok(ChnotTagUpdateReq {
                    content: e.try_get(MdwtRecord::CONTENT)?,
                    meta_otid: e.try_get(MdwtRecord::OTID)?,
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

    async fn chnot_meta(&self, req: KReq<ChnotMetaReq>) -> AResult<ChnotMetaRsp> {
        let conn = self.conn().await?;
        let chnot_meta = conn
            .qry_one(
                ChnotMetadata::pkey_reader(req.chnot_meta_otid),
                |e| ChnotMetadata::try_from(&e),
                false,
            )
            .await?;

        let block_metas = conn
            .qry_list(
                SqlBuilder::read_all(ChnotBlockMeta::TABLE).r#where(Wheres::equal(
                    ChnotBlockMeta::CHNOT_OTID,
                    req.chnot_meta_otid,
                )),
                |row| ChnotBlockMeta::try_from(&row),
            )
            .await?;
        /*         let toents = conn
        .qry_list(
            SqlBuilder::read_all(ChnotBlockToent::TABLE).r#where(Wheres::equal(
                ChnotBlockToent::CHNOT_OTID,
                req.chnot_meta_otid,
            )),
            |row| ChnotBlockToent::try_from(&row),
        )
        .await?; */

        Ok(ChnotMetaRsp {
            chnot_meta,
            block_meta_sorted: block_metas,
        })
    }

    async fn mdwt_blocks(&self, req: KReq<MdwtBlocksReq>) -> AResult<MdwtBlocksRsp> {
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

        Ok(MdwtBlocksRsp {
            mdwt_map: recs.into_iter().map(|r| (r.otid, r)).collect(),
        })
    }
}

impl TryFrom<&KDbRow> for ChnotMetadata {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = ChnotMetadata {
            otid: value.try_get(ChnotMetadata::OTID)?,
            kspace: value.try_get(ChnotMetadata::KSPACE)?,
            pin_time: value.try_get(ChnotMetadata::PIN_TIME)?,
            archive_time: value.try_get(ChnotMetadata::ARCHIVE_TIME)?,
            tid: value.try_get(ChnotMetadata::TID)?,
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

impl TryFrom<&KDbRow> for ChnotTag {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = ChnotTag {
            tid: value.try_get(ChnotTag::TID)?,
            kspace: value.try_get(ChnotTag::KSPACE)?,
            tag: value.try_get(ChnotTag::TAG)?,
            meta_otid: value.try_get(ChnotTag::META_OTID)?,
        };
        Ok(obj)
    }
}
