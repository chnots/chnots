pub(crate) mod inner;

use std::str::FromStr;

use super::mapper::ChnotMapper;
use super::*;
use crate::krate::toent::logic::todoevent::TodoEvent;
use crate::mapper::db::helper::create_tables;
use crate::mapper::db::{
    KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;
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
    .sov("SELECT r.tid as rec_tid, r.content, r.omit_tid as rec_omit_tid, r.archor,")
    .sov("m.otid as meta_otid, m.kspace, m.kind, m.pin_time, m.omit_tid as meta_omit_tid, m.archive_time, r.todo_event, m.tid as meta_tid")
    .sov("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_otid = m.otid")
}

impl ChnotTag {
    fn with_those_tag_meta_otids<'a>(
        kspaces: Vec<Varchar<40>>,
        omit_tid: OmitTID,
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
                Wheres::equal(ChnotTag::OMIT_TID, omit_tid),
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
    log::debug!("begin to build chnot");
    let record = ChnotRecord {
        tid: row.try_get("rec_tid")?,
        meta_otid: row.try_get("meta_otid")?,
        content: row.try_get("content")?,
        omit_tid: row.try_get("rec_omit_tid")?,
        archor: row.try_get("archor")?,
        todo_event: {
            let opt: Option<String> = row.try_get("todo_event")?;
            match opt {
                Some(opt) => Some(TodoEvent::from_str(opt.as_str())?),
                None => None,
            }
        },
    };
    let meta = ChnotMetadata {
        otid: row.try_get("meta_otid")?,
        kspace: row.try_get("kspace")?,
        kind: row.try_get("kind")?,
        pin_time: row.try_get("pin_time")?,
        omit_tid: row.try_get("meta_omit_tid")?,
        archive_time: row.try_get("archive_time")?,
        tid: row.try_get("meta_tid")?,
    };
    Ok(Chnot { record, meta })
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
                OmitTID::never(),
                req.tags.as_ref(),
            ))
            .sov(")")
            .merge(
                SqlBuilder::read(ChnotTag::TABLE, &[field])
                    .sov("as t")
                    .sov("right join qualified_tids q on t.meta_otid = q.meta_otid")
                    .r#where(Wheres::and([
                        Wheres::r#in(ChnotTag::KSPACE, req.get_spaces()),
                        Wheres::equal(ChnotTag::OMIT_TID, OmitTID::never()),
                    ]))
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
                ChnotTag::create_sql(),
                ChnotMetadata::create_sql(),
                ChnotRecord::create_sql(),
                ChnotKindRel::create_sql(),
            ],
            self,
        )
        .await
    }

    async fn chnot_archive(&self, req: KReq<ChnotArchiveReq>) -> AResult<ChnotArchiveRsp> {
        self.conn()
            .await?
            .exec(
                ChnotMetadata::pkey_updater(req.meta_otid, OmitTID::never())
                    .set(ChnotMetadata::ARCHIVE_TIME, Local::now().fixed_offset()),
            )
            .await?;

        Ok(ChnotArchiveRsp {})
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
                        ChnotTag::with_those_tag_meta_otids(
                            req.get_spaces(),
                            OmitTID::never(),
                            Some(tag),
                        ),
                    )
                    .sov("on t.meta_otid = ct.meta_otid")
            })
            .r#where(Wheres::and([
                // default without omit chnot record
                // TODO: group by perm tid
                Wheres::transform(req.with_omitted, |e| {
                    Wheres::compare(
                        "meta_omit_tid",
                        if e.default_false() { "<" } else { "=" },
                        OmitTID::never(),
                    )
                }),
                Wheres::transform(req.with_omitted, |e| {
                    Wheres::compare(
                        "rec_omit_tid",
                        if e.default_false() { "<" } else { "=" },
                        OmitTID::never(),
                    )
                }),
                Wheres::transform(req.with_archive, |e| {
                    if e.default_false() {
                        Wheres::None
                    } else {
                        Wheres::is_null("archive_time")
                    }
                }),
                Wheres::transform(&req.kinds, |k| {
                    if !k.is_empty() {
                        Wheres::r#in("t.kind", k.iter().map(|e| e.to_string()).collect())
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

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        let mut conn = self.conn().await?;

        let tx = conn.tx().await?;
        let reader = ChnotMetadata::pkey_reader(req.meta_otid, OmitTID::never());
        let meta: Option<ChnotMetadata> = tx.qry_opt(reader, |e| e.try_into()).await?;

        let Some(mut meta) = meta else {
            return Err(anyhow!("unable to file this chnot meta, {:?}", req));
        };
        meta.omit_tid = OmitTID::now();

        tx.exec(meta.to_sql_inserter()).await?;

        let omit = ChnotMetadata::pkey_updater(req.meta_otid, OmitTID::never())
            .set_if_some(
                ChnotMetadata::PIN_TIME,
                if let Some(o) = req.pinned {
                    if o {
                        Some(Some(Local::now().fixed_offset()))
                    } else {
                        Some(None)
                    }
                } else {
                    None
                },
            )
            .set_if_some(
                ChnotMetadata::ARCHIVE_TIME,
                if let Some(o) = req.archive {
                    if o {
                        Some(Some(Local::now().fixed_offset()))
                    } else {
                        Some(None)
                    }
                } else {
                    None
                },
            )
            .set_if_some(ChnotMetadata::KSPACE, req.body.kspace)
            .set(ChnotMetadata::TID, TID::default());

        tx.exec(omit).await?;
        tx.cmt().await?;

        Ok(ChnotUpdateRsp {})
    }

    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ans = tx.chnot_overwrite(req).await;

        if ans.is_ok() {
            tx.cmt().await?;
        } else {
            tx.rbk().await?;
        }

        ans
    }

    async fn chnot_tag_query(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<ChnotTag>> {
        self.chnot_tag_query_inner(req, |e| e.try_into(), false)
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
        let get_all = SqlBuilder::read(
            ChnotRecord::TABLE,
            &[ChnotRecord::CONTENT, ChnotRecord::META_OTID],
        )
        .r#where(Wheres::and([
            Wheres::equal(ChnotRecord::OMIT_TID, OmitTID::never()),
            Wheres::compare_str(
                ChnotRecord::META_OTID,
                "in",
                format!(
                    "(select {} from {} where kspace = '{}')",
                    ChnotMetadata::OTID,
                    ChnotMetadata::TABLE,
                    kspace.as_str().replace("'", "<quote>")
                ),
            ),
        ]));

        let kspace = kspace.to_owned();
        let mut conn = self.conn().await?;
        let chnots = conn
            .qry_list(get_all, move |e| {
                Ok(ChnotTagUpdateReq {
                    content: e.try_get(ChnotRecord::CONTENT)?,
                    meta_otid: e.try_get(ChnotRecord::META_OTID)?,
                    kspace: kspace.to_owned(),
                })
            })
            .await?;

        let tx = conn.transaction().await?;
        for one in chnots {
            let mut chnot_parser = parser::ChnotParser::new(one.content.as_str());
            chnot_parser.parse();
            tx.chnot_tag_update_single_chnot(one.clone(), &chnot_parser)
                .await?;
        }
        tx.cmt().await?;

        Ok(())
    }

    async fn chnot_query_kind_rel(
        &self,
        req: KReq<ChnotKindRelQueryReq>,
    ) -> AResult<ChnotKindRelQueryRsp> {
        self.conn()
            .await?
            .qry_one(
                ChnotKindRel::pkey_reader(req.meta_otid, OmitTID::never()),
                |e| e.try_into(),
                false,
            )
            .await
            .map(|e| ChnotKindRelQueryRsp { kind_rel: e })
    }
}

impl TryFrom<KDbRow> for ChnotMetadata {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let chnot = ChnotMetadata {
            otid: value.try_get(ChnotMetadata::OTID)?,
            kspace: value.try_get(ChnotMetadata::KSPACE)?,
            kind: value.try_get(ChnotMetadata::KIND)?,
            pin_time: value.try_get(ChnotMetadata::PIN_TIME)?,
            omit_tid: value.try_get(ChnotMetadata::OMIT_TID)?,
            archive_time: value.try_get(ChnotMetadata::ARCHIVE_TIME)?,
            tid: value.try_get(ChnotMetadata::TID)?,
        };
        Ok(chnot)
    }
}

impl TryFrom<KDbRow> for ChnotRecord {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let chnot = ChnotRecord {
            meta_otid: value.try_get(ChnotRecord::META_OTID)?,
            content: value.try_get(ChnotRecord::CONTENT)?,
            omit_tid: value.try_get(ChnotRecord::OMIT_TID)?,
            tid: value.try_get(ChnotRecord::TID)?,
            archor: value.try_get(ChnotRecord::ARCHOR)?,
            todo_event: {
                let opt: Option<String> = value.try_get(ChnotRecord::TODO_EVENT)?;
                match opt {
                    Some(opt) => Some(TodoEvent::from_str(opt.as_str())?),
                    None => None,
                }
            },
        };
        Ok(chnot)
    }
}

impl TryFrom<KDbRow> for ChnotTag {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let obj = ChnotTag {
            tid: value.try_get(ChnotTag::TID)?,
            kspace: value.try_get(ChnotTag::KSPACE)?,
            tag: value.try_get(ChnotTag::TAG)?,
            meta_otid: value.try_get(ChnotTag::META_OTID)?,
            omit_tid: value.try_get(ChnotTag::OMIT_TID)?,
        };
        Ok(obj)
    }
}

impl TryFrom<KDbRow> for ChnotKindRel {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        Ok(ChnotKindRel {
            meta_otid: value.try_get(ChnotKindRel::META_OTID)?,
            omit_tid: value.try_get(ChnotKindRel::OMIT_TID)?,
            kind_id: value.try_get(ChnotKindRel::KIND_ID)?,
            tid: value.try_get(ChnotKindRel::TID)?,
        })
    }
}
