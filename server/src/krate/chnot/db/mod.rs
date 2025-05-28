pub(crate) mod inner;

use super::mapper::{ChnotDeserializeMapper, ChnotDumpMapper, ChnotMapper};
use super::*;
use crate::magics::KImplWrapper;
use crate::mapper::db::tabledumpsql::TableDumpSqlBuilder;
use crate::mapper::db::{
    KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, KDbTransactionBehaiver,
};
use crate::model::dto::KReq;
use crate::util::string_util::get_hashtags;
use anyhow::Ok;
use chin_sql::{ILikeType, SqlReader, SqlValue};
use chin_sql::{LimitOffset, SqlUpdater, Wheres};
use chin_tools::{AResult, EResult};
use chrono::{Local, Utc};
use itertools::Itertools;
use serde::Serialize;
use tracing::info;

const UNTAGGED_TAG: &str = "#_untagged";

#[inline]
fn chnot_query_sql<'a>() -> SqlReader<'a> {
    SqlReader::new()
    .raw("SELECT r.id as rid, r.content, r.omit_time, r.insert_time as version_time,")
    .raw("m.id as mid, m.kspace, m.kind, m.pin_time, m.delete_time, m.update_time, m.insert_time as init_time, m.archive_time")
    .raw("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_id = m.id")
}

#[inline]
fn chnot_query_mapper(row: KDbRow) -> AResult<Chnot> {
    tracing::debug!("begin to build chnot");
    let record = ChnotRecord {
        id: row.try_get("rid")?,
        meta_id: row.try_get("mid")?,
        content: row.try_get("content")?,
        omit_time: row.try_get("omit_time")?,
        insert_time: row.try_get("version_time")?,
    };
    let meta = ChnotMetadata {
        id: row.try_get("mid")?,
        kspace: row.try_get("kspace")?,
        kind: row.try_get("kind")?,
        pin_time: row.try_get("pin_time")?,
        delete_time: row.try_get("delete_time")?,
        update_time: row.try_get("update_time")?,
        insert_time: row.try_get("init_time")?,
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
            .raw("order by tag asc")
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
                    .set(ChnotMetadata::DELETE_TIME, Local::now().fixed_offset())
                    .r#where(Wheres::equal(ChnotMetadata::ID, &req.chnot_id)),
            )
            .await?;

        Ok(ChnotDeletionRsp {})
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        let page_size = req.page_size;
        let page_start = req.start_index;

        let chnot_sql = chnot_query_sql()
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
                |tag, ss| {
                    ss.raw("inner join")
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
                        .raw("on r.meta_id = ct.chnot_meta_id")
                },
            )
            .r#where(Wheres::and([
                // default without deleted chnot
                Wheres::transform(req.with_deleted, |e| {
                    if e.unwrap_or(false) {
                        Wheres::none()
                    } else {
                        Wheres::is_null("delete_time")
                    }
                }),
                // default without omit chnot record
                // TODO: group by perm id
                Wheres::transform(req.with_omitted, |e| {
                    if e.unwrap_or(false) {
                        Wheres::none()
                    } else {
                        Wheres::is_null("omit_time")
                    }
                }),
                Wheres::transform(req.with_archived, |e| {
                    if !e.unwrap_or(false) {
                        Wheres::is_null("m.archive_time")
                    } else {
                        Wheres::none()
                    }
                }),
                Wheres::equal("m.kspace", req.kspace.clone()),
                Wheres::if_some(req.query.as_ref(), |content| {
                    Wheres::ilike("content", content, ILikeType::Fuzzy)
                }),
                // TODO how to use as_ref?
                Wheres::if_some(req.record_id.to_owned(), |id| Wheres::equal("r.id", id)),
                // TODO how to use as_ref?
                Wheres::if_some(req.meta_id.to_owned(), |id| Wheres::equal("r.meta_id", id)),
            ]))
            .raw("ORDER BY m.pin_time DESC, m.insert_time desc")
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
            .r#where(Wheres::equal("id", &req.meta_id));

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
            .sov("select r.content, m.id as meta_id from ")
            .sov(ChnotRecord::TABLE)
            .sov(" as r left join")
            .sov(ChnotMetadata::TABLE)
            .sov(" as m on r.meta_id = m.id where r.omit_time is null and kspace = ")
            .sov(SqlValue::Str(kspace.into()));

        let kspace = kspace.to_owned();
        let mut conn = self.conn().await?;
        let chnots = conn
            .qry_list(get_all, move |e| {
                Ok(ChnotTagUpdateReq {
                    content: e.try_get(ChnotRecord::CONTENT)?,
                    meta_id: e.try_get("meta_id")?,
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

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        let mut conn = self.conn().await?;
        let wrapper = KImplWrapper(conn.transaction().await?);
        wrapper.chnot_tag_delete(chnot_meta_ids).await
    }
}

impl ChnotDeserializeMapper for KDbRow {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: self.try_get(ChnotMetadata::ID)?,
            kspace: self.try_get(ChnotMetadata::KSPACE)?,
            kind: self.try_get(ChnotMetadata::KIND)?,
            pin_time: self.try_get(ChnotMetadata::PIN_TIME)?,
            delete_time: self.try_get(ChnotMetadata::DELETE_TIME)?,
            update_time: self.try_get(ChnotMetadata::UPDATE_TIME)?,
            insert_time: self.try_get(ChnotMetadata::INSERT_TIME)?,
            archive_time: self.try_get(ChnotMetadata::ARCHIVE_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: self.try_get(ChnotRecord::ID)?,
            meta_id: self.try_get(ChnotRecord::META_ID)?,
            content: self.try_get(ChnotRecord::CONTENT)?,
            omit_time: self.try_get(ChnotRecord::OMIT_TIME)?,
            insert_time: self.try_get(ChnotRecord::INSERT_TIME)?,
        };
        Ok(chnot)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            id: self.try_get(ChnotTag::ID)?,
            kspace: self.try_get(ChnotTag::KSPACE)?,
            tag: self.try_get(ChnotTag::TAG)?,
            chnot_meta_id: self.try_get(ChnotTag::CHNOT_META_ID)?,
            insert_time: self.try_get(ChnotTag::INSERT_TIME)?,
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
