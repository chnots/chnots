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
};
use chin_sql::{ILikeType, SqlDeleter, SqlInserter, SqlReader};
use chin_tools::wrapper::anyhow::{AResult, EResult};
use chrono::Local;
use itertools::Itertools;
use serde::Serialize;

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
    fn chnot_tag_filter<'a>(tst: TagSearchType, query: String) -> Wheres<'a> {
        match tst {
            TagSearchType::Exact => Wheres::ilike("tag", query, chin_sql::ILikeType::Original),
            TagSearchType::Fuzzy => Wheres::ilike("tag", query, chin_sql::ILikeType::Fuzzy),
            _ => Wheres::ilike(
                "tag",
                if query.len() > 0 { query + "/" } else { query },
                chin_sql::ILikeType::RightFuzzy,
            ),
        }
    }

    async fn chnot_tag_query_inner<F, T>(
        &self,
        req: KReq<ChnotTagQueryReq>,
        mapper: F,
        name_only: bool,
    ) -> AResult<ChnotTagQueryRsp<T>>
    where
        F: Fn(KDbRow<'_>) -> AResult<T> + Send + 'static,
        T: Serialize + Clone + Send + 'static,
    {
        let ChnotTagQueryReq {
            query,
            page_size,
            start_index,
            query_type,
        } = req.body;
        let ns = req.namespace;

        let sr = if name_only {
            SqlReader::read(ChnotTag::TABLE, &[ChnotTag::TAG])
        } else {
            SqlReader::read_all(ChnotTag::TABLE)
        };

        let query = sr
            .r#where(Wheres::and([
                Wheres::if_some(query, |query| Self::chnot_tag_filter(query_type, query)),
                Wheres::equal("namespace", ns),
            ]))
            .raw("order by tag asc")
            .custom(LimitOffset::new(page_size).offset(start_index));

        let data = self
            .conn()
            .await?
            .qry_list(query, move |e| mapper(e))
            .await?;

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
        let conn = self.conn().await?;

        let chnot_sql = chnot_query_sql()
            .some_then(
                match &req.view_type {
                    ChnotViewType::Timeline => None,
                    ChnotViewType::TagTree(chnot_view_tag_tree) => Some(chnot_view_tag_tree),
                },
                |tag, ss| {
                    ss.raw("inner join")
                        .sub(
                            "ct",
                            SqlReader::read_all(ChnotTag::TABLE).r#where(Wheres::and([
                                Wheres::equal(ChnotTag::NAMESPACE, req.namespace.to_owned()),
                                match tag {
                                    ChnotViewTagTree::OneLayer(path) => Wheres::and([
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
                                    ChnotViewTagTree::Full(path) => {
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
                // TODO how to use as_ref?
                Wheres::if_some(req.record_id.to_owned(), |id| Wheres::equal("r.id", id)),
                // TODO how to use as_ref?
                Wheres::if_some(req.meta_id.to_owned(), |id| Wheres::equal("r.meta_id", id)),
            ]))
            .raw("ORDER BY m.pin_time DESC, m.insert_time desc")
            .custom(LimitOffset::new(req.page_size).offset_if_some(Some(req.start_index)));

        let cs = conn.qry_list(chnot_sql, chnot_query_mapper).await?;

        Ok(ChnotQueryRsp {
            data: cs,
            start_index: req.start_index,
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
        let count = |s: &str| {
            if s.is_empty() {
                return -1;
            }
            s.chars().filter(|c| *c == '/').count() as i32
        };
        let query_type = req.query_type.clone();
        let tag = req.query.clone();
        let origin_count = req.body.query.as_ref().map_or(-1, |e| count(e));
        let mut result: ChnotTagQueryRsp<String> = self
            .chnot_tag_query_inner(req, |e| e.try_get(ChnotTag::TAG), true)
            .await?;

        result.data = result.data.into_iter().unique().collect();

        if let TagSearchType::OneLevel = query_type {
            let data = result
                .data
                .into_iter()
                .filter(|e| count(e) == origin_count + 1)
                .collect();

            result.data = data;
        }

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
}
