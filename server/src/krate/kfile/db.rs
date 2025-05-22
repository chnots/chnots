use super::{mapper::KFileMapper, *};
use crate::{mapper::db::KDbRow, model::dto::KReq};
use chin_tools::{AResult, EResult};
use chrono::{DateTime, FixedOffset, TimeDelta};

use super::mapper::KFileDeserializeMapper;
use crate::mapper::db::{KDb, KDbBehaiver, KDbConnBehaiver, KDbRowBehavier};

use chin_sql::{LimitOffset, OnConflict, SqlDeleter, SqlInserter, SqlReader, Wheres};

impl KFileDeserializeMapper for KDbRow<'_> {
    fn to_inline_kfile(self) -> AResult<InlineKFile> {
        let obj = InlineKFile {
            id: self.try_get(InlineKFile::ID)?,
            name: self.try_get(InlineKFile::NAME)?,
            content: self.try_get(InlineKFile::CONTENT)?,
            content_type: self.try_get(InlineKFile::CONTENT_TYPE)?,
            delete_time: self.try_get(InlineKFile::DELETE_TIME)?,
            insert_time: self.try_get(InlineKFile::INSERT_TIME)?,
            workspace: self.try_get(InlineKFile::WORKSPACE)?,
            rid: self.try_get(InlineKFile::RID)?,
            archor: self.try_get(InlineKFile::ARCHOR)?,
        };
        Ok(obj)
    }

    fn to_kfile(self) -> AResult<KFile> {
        let obj = KFile {
            id: self.try_get(KFile::ID)?,
            insert_time: self.try_get(KFile::INSERT_TIME)?,
            delete_time: self.try_get(KFile::DELETE_TIME)?,
            workspace: self.try_get(KFile::WORKSPACE)?,
            ori_filename: self.try_get(KFile::ORI_FILENAME)?,
            content_type: self.try_get(KFile::CONTENT_TYPE)?,
            ori_last_modified: self.try_get(KFile::ORI_LAST_MODIFIED)?,
            filesize: self.try_get(KFile::FILESIZE)?,
        };
        Ok(obj)
    }
}

impl KFileMapper for KDb {
    async fn ensure_table_kfile(&self) -> EResult {
        self.ensure_table_inline_kfile().await?;
        self.create_table(KFile::schema(self.db_type())).await
    }

    async fn ensure_table_inline_kfile(&self) -> EResult {
        self.create_table(InlineKFile::schema(self.db_type())).await
    }

    async fn insert_kfile(&self, res: &KFile) -> AResult<KFile> {
        let KFile {
            ori_filename,
            id,
            content_type,
            workspace,
            delete_time: _,
            insert_time: _,
            filesize,
            ori_last_modified,
        } = res;

        let conn = self.conn().await?;

        let insert_time = chrono::Utc::now().to_owned().fixed_offset();

        conn.exec(
            SqlInserter::new(KFile::TABLE)
                .fields(KFile::ID, id.to_owned())
                .fields(KFile::ORI_FILENAME, ori_filename.to_owned())
                .fields(KFile::WORKSPACE, workspace.to_owned())
                .fields(KFile::CONTENT_TYPE, content_type.to_owned())
                .fields(KFile::INSERT_TIME, insert_time.to_owned())
                .fields(KFile::FILESIZE, *filesize)
                .fields(KFile::ORI_LAST_MODIFIED, *ori_last_modified),
        )
        .await
        .map(|_| KFile {
            id: id.to_owned(),
            workspace: workspace.to_owned(),
            ori_filename: ori_filename.to_string(),
            content_type: content_type.to_owned(),
            insert_time: insert_time.fixed_offset(),
            delete_time: None,
            filesize: *filesize,
            ori_last_modified: *ori_last_modified,
        })
    }

    async fn query_kfile_by_id(&self, id: &str) -> AResult<KFile> {
        let conn = self.conn().await?;
        let res = conn
            .qry_one(
                SqlReader::read_all(KFile::TABLE).r#where(Wheres::equal(KFile::ID, id)),
                |e| e.to_kfile(),
                false,
            )
            .await?;
        Ok(res)
    }

    async fn insert_inline_kfile(
        &self,
        req: &KReq<InsertInlineKFileReq>,
    ) -> anyhow::Result<InsertInlineKFileRsp> {
        let delete_sql = SqlDeleter::new(InlineKFile::TABLE).r#where(Wheres::and([
            Wheres::equal(InlineKFile::RID, &req.res.rid),
            Wheres::equal(InlineKFile::ARCHOR, false),
        ]));

        let last_archor_sql = SqlReader::read(InlineKFile::TABLE, &[InlineKFile::INSERT_TIME])
            .r#where(Wheres::and([
                Wheres::equal(InlineKFile::RID, &req.res.rid),
                Wheres::equal(InlineKFile::ARCHOR, true),
            ]))
            .limit(1);

        self.conn().await?.exec(delete_sql).await?;
        let last_archor: Option<DateTime<FixedOffset>> = self
            .conn()
            .await?
            .qry_opt(last_archor_sql, |e| e.try_get(InlineKFile::INSERT_TIME))
            .await?;

        let archorp = match last_archor {
            Some(last) => {
                req.res.insert_time.signed_duration_since(last)
                    > TimeDelta::seconds(req.archor_intervals)
            }
            None => true,
        };

        self.conn()
            .await?
            .exec(
                SqlInserter::new(InlineKFile::TABLE)
                    .fields(InlineKFile::ID, &req.res.id)
                    .fields(InlineKFile::RID, &req.res.rid)
                    .fields(InlineKFile::NAME, &req.res.name)
                    .fields(InlineKFile::CONTENT, &req.res.content)
                    .fields(InlineKFile::CONTENT_TYPE, &req.res.content_type)
                    .fields(InlineKFile::INSERT_TIME, &req.res.insert_time)
                    .fields(InlineKFile::WORKSPACE, &req.res.workspace)
                    .fields(InlineKFile::ARCHOR, archorp)
                    .on_conflict({
                        match req.ignore_conflict.as_ref() {
                            Some(ic) => {
                                if *ic {
                                    OnConflict::Ignore
                                } else {
                                    OnConflict::Default
                                }
                            }
                            None => OnConflict::Default,
                        }
                    }),
            )
            .await?;

        Ok(InsertInlineKFileRsp {})
    }

    async fn query_inline_kfile(
        &self,
        req: KReq<QueryInlineKFileReq>,
    ) -> anyhow::Result<QueryInlineKFileRsp> {
        let query = SqlReader::read_all(InlineKFile::TABLE)
            .r#where(Wheres::and([
                Wheres::if_some(req.content_type.to_owned(), |e| {
                    Wheres::equal(InlineKFile::CONTENT_TYPE, e)
                }),
                Wheres::if_some(req.id.to_owned(), |e| Wheres::equal(InlineKFile::ID, e)),
                Wheres::if_some(req.name_like.to_owned(), |e| {
                    Wheres::ilike(InlineKFile::NAME, e, chin_sql::ILikeType::Fuzzy)
                }),
                Wheres::if_some(
                    match req.with_del {
                        Some(true) => None,
                        _ => Some(()),
                    },
                    |_| Wheres::is_null(InlineKFile::DELETE_TIME),
                ),
                Wheres::if_some(req.rid.to_owned(), |id| Wheres::equal(InlineKFile::RID, id)),
            ]))
            .raw("order by insert_time desc")
            .custom(LimitOffset::new(1));

        let res = self
            .conn()
            .await?
            .qry_list(query, |t| t.to_inline_kfile())
            .await?;

        Ok(QueryInlineKFileRsp { res })
    }
}
