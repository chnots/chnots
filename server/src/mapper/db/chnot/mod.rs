pub(crate) mod inserter;

use super::{
    sql::{LimitOffset, SqlUpdater, Wheres},
    KDb,
};
use crate::{
    mapper::{
        db::{KDbBehaiver, KDbConnBehaiver, KDbRow},
        ChnotMapper, DeserializeMapper,
    },
    model::{
        db::chnot::{ChnotMetadata, ChnotRecord, ChnotTag, ChnotTagType},
        dto::KReq,
    },
    util::string_util::get_hashtags,
};
use chin_sql::{ILikeType, SqlDeleter, SqlInserter, SqlReader, SqlValue};
use chin_tools::{
    utils::id_util,
    wrapper::anyhow::{AResult, EResult},
};
use chrono::{Local, Utc};
use itertools::Itertools;
use serde::Serialize;
use tracing::info;

use crate::model::dto::chnot::*;

const UNTAGGED_TAG: &str = "#_untagged";

#[inline]
fn chnot_query_sql<'a>() -> SqlReader<'a> {
    SqlReader::new()
    .raw("SELECT r.id as rid, r.content, r.omit_time, r.insert_time as version_time,")
    .raw("m.id as mid, m.namespace, m.kind, m.pin_time, m.delete_time, m.update_time, m.insert_time as init_time, m.archive_time")
    .raw("FROM chnot_record r LEFT JOIN chnot_metadata m ON r.meta_id = m.id")
}

#[inline]
fn chnot_query_mapper(row: KDbRow<'_>) -> AResult<Chnot> {
    tracing::debug!("begin to build chnot");
    let record = ChnotRecord {
        id: row.try_get("rid")?,
        meta_id: row.try_get("mid")?,
        content: row.try_get("content")?,
        omit_time: row.try_get_df_opt("omit_time").map(|e| e.into())?,
        insert_time: row.try_get_df("version_time")?,
    };
    let meta = ChnotMetadata {
        id: row.try_get("mid")?,
        namespace: row.try_get("namespace")?,
        kind: row.try_get("kind")?,
        pin_time: row.try_get_df_opt("pin_time")?,
        delete_time: row.try_get_df_opt("delete_time")?,
        update_time: row.try_get_df_opt("update_time")?,
        insert_time: row.try_get_df("init_time")?,
        archive_time: row.try_get_df_opt("archive_time")?,
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
        F: Fn(KDbRow<'_>) -> AResult<T> + Send + 'static,
        T: Serialize + Clone + Send + 'static + AsRef<str>,
    {
        let ChnotTagQueryReq {
            query,
            page_size,
            start_index,
            tag_tree,
        } = req.body;
        let ns = req.namespace;

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
                        if prefix.len() > 0 {
                            tag_str.push_str(&prefix);
                            tag_str.push('/');
                        }
                        tag_str.push('%');
                        if let Some(fuzzy) = query {
                            tag_str.push_str(&fuzzy);
                            tag_str.push_str("%");
                        }

                        tag_str
                    },
                    chin_sql::ILikeType::Original,
                ),
                Wheres::equal("namespace", ns),
            ]))
            .raw("order by tag asc")
            .custom(LimitOffset::new(page_size).offset(start_index));

        let mut data = self
            .conn()
            .await?
            .qry_list(query, move |e| mapper(e))
            .await?;

        if let ChnotTagTreeType::Children(prefix) = query_type1 {
            data = data
                .into_iter()
                .filter(|tag| {
                    tag.as_ref().starts_with(&prefix) && level(tag.as_ref()) == origin_count + 1
                })
                .collect();
        }

        Ok(ChnotTagQueryRsp { data, start_index })
    }
}

impl ChnotMapper for KDb {
    async fn ensure_table_chnot_record(&self) -> EResult {
        self.create_table(ChnotRecord::schema(self.db_type())).await
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        self.create_table(ChnotMetadata::schema(self.db_type()))
            .await
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        let client = self.conn().await?;

        client
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
        let conn = self.conn().await?;

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
                                Wheres::equal(ChnotTag::NAMESPACE, req.namespace.to_owned()),
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
                Wheres::equal("m.namespace", req.namespace.clone()),
                Wheres::if_some(req.query.as_ref(), |content| {
                    Wheres::ilike("content", content, ILikeType::Fuzzy)
                }),
                Wheres::equal("m.kind", "mdwt"),
                // TODO how to use as_ref?
                Wheres::if_some(req.record_id.to_owned(), |id| Wheres::equal("r.id", id)),
                // TODO how to use as_ref?
                Wheres::if_some(req.meta_id.to_owned(), |id| Wheres::equal("r.meta_id", id)),
            ]))
            .raw("ORDER BY m.pin_time DESC, m.insert_time desc")
            .custom(LimitOffset::new(req.page_size).offset_if_some(Some(req.start_index)));

        let cs = conn.qry_list(chnot_sql, chnot_query_mapper).await?;

        Ok(ChnotQueryRsp {
            has_next: cs.len() >= page_size as usize,
            data: cs,
            next_start: page_start + page_size,
        })
    }

    async fn chnot_update(&self, req: KReq<ChnotUpdateReq>) -> AResult<ChnotUpdateRsp> {
        let client = self.conn().await?;

        let su = SqlUpdater::new("chnot_metadata")
            .set_if_some("pinned", req.pinned)
            .set_if_some(
                "archive_time",
                req.archive.map(|_| Local::now().fixed_offset()),
            )
            .set_if_some("namespace", req.body.namespace.as_ref())
            .r#where(Wheres::equal("id", &req.meta_id).into());

        client.exec(su).await?;

        Ok(ChnotUpdateRsp {})
    }

    async fn ensure_table_chnot_tag(&self) -> EResult {
        self.create_table(ChnotTag::schema(self.db_type())).await
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

    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult {
        self.conn()
            .await?
            .exec(
                SqlInserter::new(ChnotTag::TABLE)
                    .fields(ChnotTag::ID, &req.id)
                    .fields(ChnotTag::CHNOT_META_ID, &req.chnot_meta_id)
                    .fields(ChnotTag::NAMESPACE, &req.namespace)
                    .fields(ChnotTag::TAG, &req.tag)
                    .fields(ChnotTag::CATEGORY, req.category)
                    .fields(ChnotTag::INSERT_TIME, req.insert_time),
            )
            .await?;

        Ok(())
    }

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        let client = self.conn().await?;
        for e in chnot_meta_ids {
            client
                .exec(
                    SqlDeleter::new(ChnotTag::TABLE)
                        .r#where(Wheres::equal(ChnotTag::CHNOT_META_ID, e)),
                )
                .await?;
        }

        Ok(())
    }

    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        self.chnot_overwrite(req).await
    }

    async fn chnot_tag_update_single_chnot(
        &self,
        content: &str,
        meta_id: &str,
        namespace: &str,
    ) -> EResult {
        self.chnot_tag_delete(vec![&meta_id]).await?;

        let tags = get_hashtags(&content);
        let parent_tags: Vec<&str> = tags
            .iter()
            .map(|tag| {
                let mut more = vec![];
                for (size, c) in tag.char_indices() {
                    if c == '/' {
                        more.push(&tag[..size]);
                    }
                }
                more
            })
            .flatten()
            .unique()
            .collect();

        if parent_tags.is_empty() {
            self.chnot_tag_insert(ChnotTag {
                id: id_util::generate_uuid(),
                namespace: namespace.to_owned(),
                tag: UNTAGGED_TAG.to_owned(),
                chnot_meta_id: meta_id.to_string(),
                insert_time: Utc::now().fixed_offset(),
                category: ChnotTagType::Dir,
            })
            .await?;
        }
        for tag in parent_tags {
            self.chnot_tag_insert(ChnotTag {
                id: id_util::generate_uuid(),
                namespace: namespace.to_owned(),
                tag: tag.to_owned(),
                chnot_meta_id: meta_id.to_string(),
                insert_time: Utc::now().fixed_offset(),
                category: ChnotTagType::ParentDir,
            })
            .await?;
        }
        for tag in tags {
            self.chnot_tag_insert(ChnotTag {
                id: id_util::generate_uuid(),
                namespace: namespace.to_owned(),
                tag: tag.to_owned(),
                chnot_meta_id: meta_id.to_string(),
                insert_time: Utc::now().fixed_offset(),
                category: ChnotTagType::Dir,
            })
            .await?;
        }

        Ok(())
    }

    async fn chnot_tag_update_all(&self, namespace: &str) -> EResult {
        let get_all = SqlReader::new()
            .sov("select r.content, m.id as meta_id from ")
            .sov(ChnotRecord::TABLE)
            .sov(" as r left join")
            .sov(ChnotMetadata::TABLE)
            .sov(" as m on r.meta_id = m.id where r.omit_time is null and namespace = ")
            .sov(SqlValue::Str(namespace.into()));

        let namespace = namespace.to_owned();
        let chnots = self
            .conn()
            .await?
            .qry_list(get_all, move |e| {
                Ok(ChnotTagUpdateReq {
                    content: e.try_get(ChnotRecord::CONTENT)?,
                    meta_id: e.try_get("meta_id")?,
                    namespace: namespace.to_owned(),
                })
            })
            .await?;

        for one in &chnots {
            self.chnot_tag_update_single_chnot(&one.content, &one.meta_id, &one.namespace)
                .await?;
        }

        Ok(())
    }
}
