pub(crate) mod inner;

use super::mapper::{ChnotDeserializeMapper, ChnotDumpMapper, ChnotMapper};
use super::*;
use crate::mapper::db::tabledumpsql::TableDumpSqlBuilder;
use crate::mapper::db::{
    KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;
use crate::util::result_util::UnwrapOr;
use anyhow::anyhow;
use chin_sql::str_type::Varchar;
use chin_sql::{ILikeType, SegOrVal, SqlBuilder};
use chin_sql::{LimitOffset, Wheres};
use chin_tools::{AResult, EResult};
use chrono::Local;
use itertools::Itertools;
use serde::Serialize;
use tracing::info;

const UNTAGGED_TAG: &str = "<NON>";

#[inline]
fn chnot_query_sql<'a>() -> SqlBuilder<'a> {
    SqlBuilder::new()
    .sov("SELECT r.tid as rec_tid, r.content, r.omit_tid as rec_omit_tid, r.archor,")
    .sov("m.tid as meta_tid, m.kspace, m.kind, m.pin_time, m.omit_tid as meta_omit_tid, m.archive_time")
    .sov("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_tid = m.tid")
}

impl ChnotTag {
    fn with_those_tag_meta_tids<'a>(
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
        SqlBuilder::read(ChnotTag::TABLE, &[ChnotTag::META_TID])
            .r#where(Wheres::and([
                Wheres::r#in(ChnotTag::KSPACE, kspaces),
                Wheres::equal(ChnotTag::OMIT_TID, omit_tid),
                Wheres::if_some(len, |_| Wheres::r#in(ChnotTag::TAG, tags.to_vec())),
            ]))
            .sov("group by")
            .sov(ChnotTag::META_TID)
            .some_then(len, |l, sb| {
                sb.sov("having")
                    .sov(format!("COUNT(DISTINCT {}) = ", ChnotTag::TAG))
                    .sov(SegOrVal::val(l as i64))
            })
    }
}

#[inline]
fn chnot_query_mapper(row: KDbRow) -> AResult<Chnot> {
    tracing::debug!("begin to build chnot");
    let record = ChnotRecord {
        tid: row.try_get("rec_tid")?,
        meta_tid: row.try_get("meta_tid")?,
        content: row.try_get("content")?,
        omit_tid: row.try_get("rec_omit_tid")?,
        archor: row.try_get("archor")?,
    };
    let meta = ChnotMetadata {
        tid: row.try_get("meta_tid")?,
        kspace: row.try_get("kspace")?,
        kind: row.try_get("kind")?,
        pin_time: row.try_get("pin_time")?,
        omit_tid: row.try_get("meta_omit_tid")?,
        archive_time: row.try_get("archive_time")?,
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
            .merge(ChnotTag::with_those_tag_meta_tids(
                req.get_spaces(),
                OmitTID::never(),
                req.tags.as_ref(),
            ))
            .sov(")")
            .merge(
                SqlBuilder::read(ChnotTag::TABLE, &[field])
                    .sov("as t")
                    .sov("right join qualified_tids q on t.meta_tid = q.meta_tid")
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
    async fn ensure_table_chnot_record(&self) -> EResult {
        self.conn()
            .await?
            .exec(ChnotRecord::create_sql())
            .await
            .map(|_| ())
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        self.conn().await?.exec(ChnotMetadata::create_sql()).await?;
        self.conn()
            .await?
            .exec(ChnotKindRel::create_sql())
            .await
            .map(|_| ())
    }

    async fn chnot_archive(&self, req: KReq<ChnotArchiveReq>) -> AResult<ChnotArchiveRsp> {
        self.conn()
            .await?
            .exec(
                ChnotMetadata::pkey_updater(req.meta_tid, OmitTID::never())
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
                        ChnotTag::with_those_tag_meta_tids(
                            req.get_spaces(),
                            OmitTID::never(),
                            Some(tag),
                        ),
                    )
                    .sov("on t.meta_tid = ct.meta_tid")
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
                Wheres::if_some(req.record_tid, |tid| Wheres::equal("t.rec_tid", tid)),
                Wheres::if_some(req.meta_tid, |tid| Wheres::equal("t.meta_tid", tid)),
            ]))
            .sov("ORDER BY t.pin_time asc, t.meta_tid desc")
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
        let reader = ChnotMetadata::pkey_reader(req.meta_tid, OmitTID::never());
        let meta = tx.qry_opt(reader, KDbRow::to_chnot_meta).await?;

        let Some(mut meta) = meta else {
            return Err(anyhow!("unable to file this chnot meta, {:?}", req));
        };
        meta.omit_tid = OmitTID::now();

        tx.exec(meta.to_sql_inserter()).await?;

        let omit = ChnotMetadata::pkey_updater(req.meta_tid, OmitTID::never())
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
            .set_if_some(ChnotMetadata::KSPACE, req.body.kspace);

        tx.exec(omit).await?;
        tx.cmt().await?;

        Ok(ChnotUpdateRsp {})
    }

    async fn ensure_table_chnot_tag(&self) -> EResult {
        self.conn()
            .await?
            .exec(ChnotTag::create_sql())
            .await
            .map(|_| ())
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
        self.chnot_tag_query_inner(req, |e| e.to_chnot_tag(), false)
            .await
    }

    async fn chnot_tag_names(
        &self,
        req: KReq<ChnotTagQueryReq>,
    ) -> AResult<ChnotTagQueryRsp<String>> {
        info!("{:#?}", req);
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
            &[ChnotRecord::CONTENT, ChnotRecord::META_TID],
        )
        .r#where(Wheres::and([
            Wheres::equal(ChnotRecord::OMIT_TID, OmitTID::never()),
            Wheres::compare_str(
                ChnotRecord::META_TID,
                "in",
                format!(
                    "(select {} from {} where kspace = '{}')",
                    ChnotMetadata::TID,
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
                    meta_tid: e.try_get(ChnotRecord::META_TID)?,
                    kspace: kspace.to_owned(),
                })
            })
            .await?;

        let tx = conn.transaction().await?;
        for one in chnots {
            tx.chnot_tag_update_single_chnot(one).await?;
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
                ChnotKindRel::pkey_reader(req.meta_tid, OmitTID::never()),
                |e| e.to_chnot_kind_rel(),
                false,
            )
            .await
            .map(|e| ChnotKindRelQueryRsp { kind_rel: e })
    }
}

impl ChnotDeserializeMapper for KDbRow {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            tid: self.try_get(ChnotMetadata::TID)?,
            kspace: self.try_get(ChnotMetadata::KSPACE)?,
            kind: self.try_get(ChnotMetadata::KIND)?,
            pin_time: self.try_get(ChnotMetadata::PIN_TIME)?,
            omit_tid: self.try_get(ChnotMetadata::OMIT_TID)?,
            archive_time: self.try_get(ChnotMetadata::ARCHIVE_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            meta_tid: self.try_get(ChnotRecord::META_TID)?,
            content: self.try_get(ChnotRecord::CONTENT)?,
            omit_tid: self.try_get(ChnotRecord::OMIT_TID)?,
            tid: self.try_get(ChnotRecord::TID)?,
            archor: self.try_get(ChnotRecord::ARCHOR)?,
        };
        Ok(chnot)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            tid: self.try_get(ChnotTag::TID)?,
            kspace: self.try_get(ChnotTag::KSPACE)?,
            tag: self.try_get(ChnotTag::TAG)?,
            meta_tid: self.try_get(ChnotTag::META_TID)?,
            omit_tid: self.try_get(ChnotTag::OMIT_TID)?,
        };
        Ok(obj)
    }

    fn to_chnot_kind_rel(self) -> AResult<ChnotKindRel> {
        Ok(ChnotKindRel {
            meta_tid: self.try_get(ChnotKindRel::META_TID)?,
            omit_tid: self.try_get(ChnotKindRel::OMIT_TID)?,
            kind_id: self.try_get(ChnotKindRel::KIND_ID)?,
            tid: self.try_get(ChnotKindRel::TID)?,
        })
    }
}

impl ChnotDumpMapper for KDb {
    async fn dump_chnot_meta(&self, callback: &crate::RecordCallbackType) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(ChnotMetadata::TABLE),
            KDbRow::to_chnot_meta,
            callback,
        )
        .await?;

        Ok(())
    }

    async fn dump_chnot_record(&self, callback: &crate::RecordCallbackType) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(ChnotRecord::TABLE),
            KDbRow::to_chnot_record,
            callback,
        )
        .await?;

        Ok(())
    }

    async fn dump_chnot_tag(&self, callback: &crate::RecordCallbackType) -> EResult {
        self.read_iterator(
            TableDumpSqlBuilder::table(ChnotTag::TABLE),
            KDbRow::to_chnot_tag,
            callback,
        )
        .await?;

        Ok(())
    }
}
