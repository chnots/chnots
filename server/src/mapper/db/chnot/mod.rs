pub(crate) mod inserter;

use std::ops::Deref;

use super::{
    sql::{LimitOffset, SqlUpdater, Wheres},
    KDb,
};
use crate::{
    mapper::{
        db::{sqlite::wrapper::KDbConnBehaiverSync, KDbBehaiver, KDbConnBehaiver, KDbRow},
        ChnotMapper, DeserializeMapper,
    },
    model::{
        db::chnot::{ChnotMetadata, ChnotRecord, ChnotTag, ChnotTagType},
        dto::KReq,
    },
    util::string_util::get_hashtags,
};
use chin_sql::{SqlDeleter, SqlInserter, SqlSegBuilder};
use chin_tools::{
    utils::id_util,
    wrapper::anyhow::{AResult, EResult},
    SharedStr,
};
use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};

use crate::model::dto::chnot::*;

#[inline]
fn chnot_query_sql<'a>() -> SqlSegBuilder<'a> {
    SqlSegBuilder::new()
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

impl ChnotMapper for KDb {
    async fn ensure_table_chnot_record(&self) -> EResult {
        self.create_table(ChnotRecord::table_creation_sql(self.db_type()))
            .await
    }

    async fn ensure_table_chnot_metadata(&self) -> EResult {
        self.create_table(ChnotMetadata::table_creation_sql(self.db_type()))
            .await
    }

    async fn chnot_delete(&self, req: KReq<ChnotDeletionReq>) -> AResult<ChnotDeletionRsp> {
        let client = self.conn().await?;

        client
            .exec(
                SqlUpdater::new(ChnotMetadata::table_name())
                    .set(
                        ChnotMetadata::field_delete_time(),
                        Local::now().fixed_offset(),
                    )
                    .r#where(Wheres::equal(ChnotMetadata::field_id(), &req.chnot_id)),
            )
            .await?;

        Ok(ChnotDeletionRsp {})
    }

    async fn chnot_query(&self, req: KReq<ChnotQueryReq>) -> AResult<ChnotQueryRsp<Vec<Chnot>>> {
        let client = self.conn().await?;

        let chnot_sql = chnot_query_sql()
            .some_then(req.tag_keyword.as_ref(), |tag, ss| {
                ss.raw("inner join")
                    .sub(
                        "ct",
                        SqlSegBuilder::new()
                            .raw("select * from chnot_tag")
                            .r#where(Wheres::and([
                                Wheres::equal("namespace", req.namespace.to_owned()),
                                Wheres::ilike("tag", tag, self.db_type()),
                            ])),
                    )
                    .raw("on r.meta_id = ct.chnot_meta_id")
            })
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
                    Wheres::ilike("content", content, self.db_type())
                }),
                // TODO how to use as_ref?
                Wheres::if_some(req.record_id.to_owned(), |id| Wheres::equal("r.id", id)),
                // TODO how to use as_ref?
                Wheres::if_some(req.meta_id.to_owned(), |id| Wheres::equal("r.meta_id", id)),
            ]))
            .raw("ORDER BY m.pin_time DESC, m.insert_time desc")
            .custom(LimitOffset::new(req.page_size).offset_if_some(Some(req.start_index)));

        let cs = client.qry_list(chnot_sql, chnot_query_mapper).await?;

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
        self.create_table(ChnotTag::table_creation_sql(self.db_type()))
            .await
    }

    async fn chnot_tag_query(&self, req: KReq<ChnotTagQueryReq>) -> AResult<ChnotTagQueryRsp> {
        let ChnotTagQueryReq {
            query,
            page_size,
            start_index,
        } = req.body;
        let ns = req.namespace;
        let query = SqlSegBuilder::new()
            .raw("select * from chnot_tag")
            .r#where(Wheres::and([
                Wheres::if_some(query, |query| Wheres::ilike("tag", query, self.db_type())),
                Wheres::equal("namespace", ns),
            ]))
            .raw("order by insert_time desc")
            .custom(LimitOffset::new(page_size).offset(start_index));

        let data = self
            .conn()
            .await?
            .qry_list(query, |e| e.to_chnot_tag())
            .await?;

        Ok(ChnotTagQueryRsp { data, start_index })
    }

    async fn chnot_tag_names(&self, req: KReq<ChnotTagQueryReq>) -> AResult<ChnotTagNamesRsp> {
        let ChnotTagQueryReq {
            query,
            page_size,
            start_index,
        } = req.body;
        let ns = req.namespace;

        let query = SqlSegBuilder::new()
            .raw("select distinct tag from chnot_tag")
            .r#where(Wheres::and([
                Wheres::if_some(query, |query| Wheres::ilike("tag", query, self.db_type())),
                Wheres::equal("namespace", ns),
            ]))
            .raw("order by tag asc")
            .custom(LimitOffset::new(page_size).offset(start_index));

        let res = self
            .conn()
            .await?
            .qry_list(query, |e| e.try_get(ChnotTag::field_tag()))
            .await;

        Ok(ChnotTagNamesRsp {
            data: res?,
            start_index,
        })
    }

    async fn chnot_tag_insert(&self, req: ChnotTag) -> EResult {
        self.conn()
            .await?
            .exec(
                SqlInserter::new(ChnotTag::table_name())
                    .fields(ChnotTag::field_id(), &req.id)
                    .fields(ChnotTag::field_chnot_meta_id(), &req.chnot_meta_id)
                    .fields(ChnotTag::field_namespace(), &req.namespace)
                    .fields(ChnotTag::field_tag(), &req.tag)
                    .fields(ChnotTag::field_category(), req.category)
                    .fields(ChnotTag::field_insert_time(), req.insert_time),
            )
            .await?;

        Ok(())
    }

    async fn chnot_tag_delete(&self, chnot_meta_ids: Vec<&str>) -> EResult {
        let client = self.conn().await?;
        for e in chnot_meta_ids {
            client
                .exec(
                    SqlDeleter::new(ChnotTag::table_name())
                        .r#where(Wheres::equal(ChnotTag::field_chnot_meta_id(), e)),
                )
                .await?;
        }

        Ok(())
    }

    async fn chnot_overwrite(&self, req: KReq<ChnotOverwriteReq>) -> AResult<ChnotOverwriteRsp> {
        self.chnot_overwrite(req).await
    }
}
