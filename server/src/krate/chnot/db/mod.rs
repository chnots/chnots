pub(crate) mod inner;

use super::mapper::{ChnotDeserializeMapper, ChnotDumpMapper, ChnotMapper};
use super::*;
use crate::magics::KImplWrapper;
use crate::mapper::db::tabledumpsql::TableDumpSqlBuilder;
use crate::mapper::db::{
    KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::model::omit_tid::OmitTID;
use crate::util::result_util::UnwrapOr;
use crate::util::string_util::get_hashtags;
use chin_sql::{ILikeType, SqlReader, SqlValue};
use chin_sql::{LimitOffset, SqlUpdater, Wheres};
use chin_tools::time_type::TID;
use chin_tools::{AResult, EResult};
use chrono::Local;
use itertools::Itertools;
use serde::Serialize;
use tracing::info;

const UNTAGGED_TAG: &str = "#_untagged";

#[inline]
fn chnot_query_sql<'a>() -> SqlReader<'a> {
    SqlReader::new()
    .sov("SELECT r.tid as rec_tid, r.content, r.omit_tid as rec_omit_tid, r.archor,")
    .sov("m.tid as meta_tid, m.kspace, m.kind, m.pin_time, m.omit_tid as meta_omit_tid, m.archive_time")
    .sov("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_tid = m.tid")
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
        let ChnotTagQueryReq {
            query,
            page_size,
            start_index,
            tag_tree,
        } = req.body;
        let ns = req.kspace;

        let level = |s: &str| {
            if s.is_empty() {
                return -1;
            }
            s.char_indices().filter(|(_, c)| *c == '/').count() as i32
        };
        let query_type1 = tag_tree.clone();

        let origin_count = level(query_type1.path());
        info!("original count: {}", origin_count);
        let sr = if name_only {
            SqlReader::read(ChnotTag::TABLE, &["distinct tag"])
        } else {
            SqlReader::read_all(ChnotTag::TABLE)
        };

        let query = sr
            .r#where(Wheres::and([
                Wheres::ilike(
                    "tag",
                    {
                        let mut tag_str = String::new();
                        let prefix = tag_tree.path();
                        if !prefix.is_empty() {
                            tag_str.push_str(prefix);
                            tag_str.push('/');
                        }
                        tag_str.push('%');
                        if let Some(fuzzy) = query {
                            tag_str.push_str(&fuzzy);
                            tag_str.push('%');
                        }

                        tag_str
                    },
                    chin_sql::ILikeType::Original,
                ),
                Wheres::equal("kspace", ns),
            ]))
            .sov("order by tag asc")
            .custom(LimitOffset::new(page_size).offset(start_index));

        let mut data = self.conn().await?.qry_list(query, mapper).await?;

        if let ChnotTagTreeType::Children(prefix) = query_type1 {
            data.retain(|tag| {
                tag.as_ref().starts_with(&prefix) && level(tag.as_ref()) == origin_count + 1
            });
        }

        Ok(ChnotTagQueryRsp { data, start_index })
    }
}

impl ChnotMapper for KDb {
    async fn ensure_table_chnot_record(&self) -> EResult {
        self.conn()
            .await?
            .create_table(ChnotRecord::schema(self.db_type()))
            .await
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        self.conn()
            .await?
            .create_table(ChnotMetadata::schema(self.db_type()))
            .await
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        self.conn()
            .await?
            .exec(
                SqlUpdater::new(ChnotMetadata::TABLE)
                    .set(ChnotMetadata::OMIT_TID, Local::now().fixed_offset())
                    .r#where(Wheres::equal(ChnotMetadata::TID, req.meta_tid)),
            )
            .await?;

        Ok(ChnotDeletionRsp {})
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        let page_size = req.page_size;
        let page_start = req.start_index;

        let chnot_sql = SqlReader::new()
            .sov("select * from ")
            .sub("t", chnot_query_sql())
            .some_then(
                match &req.view_type {
                    ChnotViewType::Timeline => None,
                    ChnotViewType::TagTree(chnot_view_tag_tree) => {
                        if chnot_view_tag_tree.is_empty() {
                            None
                        } else {
                            Some(chnot_view_tag_tree)
                        }
                    }
                },
                |tag, sr| {
                    sr.sov("inner join")
                        .sub(
                            "ct",
                            SqlReader::read_all(ChnotTag::TABLE).r#where(Wheres::and([
                                Wheres::equal(ChnotTag::KSPACE, req.kspace.to_owned()),
                                match tag {
                                    ChnotTagTreeType::Children(path) => Wheres::and([
                                        Wheres::equal(
                                            ChnotTag::TAG,
                                            if path.starts_with("#") {
                                                path
                                            } else {
                                                UNTAGGED_TAG
                                            },
                                        ),
                                        Wheres::equal(ChnotTag::CATEGORY, ChnotTagType::Dir),
                                    ]),
                                    ChnotTagTreeType::Descendants(path) => {
                                        Wheres::ilike(ChnotTag::TAG, path, ILikeType::RightFuzzy)
                                    }
                                },
                            ])),
                        )
                        .sov("on t.meta_tid = ct.meta_tid")
                },
            )
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
                Wheres::if_some(req.record_tid.to_owned(), |tid| {
                    Wheres::equal("t.rec_tid", tid)
                }),
                Wheres::if_some(req.meta_tid.to_owned(), |tid| {
                    Wheres::equal("t.meta_tid", tid)
                }),
            ]))
            .sov("ORDER BY t.pin_time DESC, t.meta_tid desc")
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
        let su = SqlUpdater::new("chnot_metadata")
            .set_if_some("pinned", req.pinned)
            .set_if_some(
                "archive_time",
                req.archive.map(|_| Local::now().fixed_offset()),
            )
            .set_if_some("kspace", req.body.kspace.as_ref())
            .r#where(Wheres::equal("tid", req.meta_tid));

        self.conn().await?.exec(su).await?;

        Ok(ChnotUpdateRsp {})
    }

    async fn ensure_table_chnot_tag(&self) -> EResult {
        self.conn()
            .await?
            .create_table(ChnotTag::schema(self.db_type()))
            .await
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
        let mut result: ChnotTagQueryRsp<String> = self
            .chnot_tag_query_inner(req, |e| e.try_get(ChnotTag::TAG), true)
            .await?;

        result.data = result.data.into_iter().unique().collect();

        Ok(result)
    }

    async fn chnot_tag_update_all(&self, kspace: &str) -> EResult {
        let get_all = SqlReader::new()
            .sov("select r.content, m.tid as meta_id from ")
            .sov(ChnotRecord::TABLE)
            .sov(" as r left join")
            .sov(ChnotMetadata::TABLE)
            .sov(" as m on r.meta_id = m.tid where r.omit_time is null and kspace = ")
            .sov(SqlValue::Str(kspace.into()));

        let kspace = kspace.to_owned();
        let mut conn = self.conn().await?;
        let chnots = conn
            .qry_list(get_all, move |e| {
                Ok(ChnotTagUpdateReq {
                    content: e.try_get(ChnotRecord::CONTENT)?,
                    meta_tid: e.try_get("meta_id")?,
                    kspace: kspace.to_owned(),
                })
            })
            .await?;

        let tx = conn.transaction().await?;
        for one in chnots {
            tx.chnot_tag_update_single_chnot(one).await?;
        }

        Ok(())
    }

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<TID>) -> EResult {
        let mut conn = self.conn().await?;
        let wrapper = KImplWrapper(conn.transaction().await?);
        wrapper.chnot_tag_delete(chnot_meta_ids).await
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
            category: ChnotTagType::Common,
        };
        Ok(obj)
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
