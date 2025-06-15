use std::io::Write;

use super::{mapper::KFileMapper, *};
use crate::{
    krate::kkv::{KKVOverwriteReq, KKVQueryOneReq},
    mapper::db::{KDbConnBehaiver, KDbExecutor, KDbRow, KDbTransactionBehaiver},
    model::{dto::KReq, omit_tid::OmitTID},
    util::result_util::UnwrapOr,
};
use chin_tools::{AResult, EResult};

use super::mapper::KFileDeserializeMapper;
use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier};

use chin_sql::{time_type::TID, LimitOffset, OnConflict, SqlBuilder, Wheres};

impl KFileDeserializeMapper for KDbRow {
    fn to_inline_kfile(self) -> AResult<InlineKFile> {
        let obj = InlineKFile {
            tid: self.try_get(InlineKFile::TID)?,
            name: self.try_get(InlineKFile::NAME)?,
            content: self.try_get(InlineKFile::CONTENT)?,
            content_type: self.try_get(InlineKFile::CONTENT_TYPE)?,
            sid: self.try_get(InlineKFile::SID)?,
        };
        Ok(obj)
    }

    fn to_kfile(self) -> AResult<KFile> {
        let obj = KFile {
            tid: self.try_get(KFile::TID)?,
            ori_filename: self.try_get(KFile::ORI_FILENAME)?,
            content_type: self.try_get(KFile::CONTENT_TYPE)?,
            ori_last_modified: self.try_get(KFile::ORI_LAST_MODIFIED)?,
            filesize: self.try_get(KFile::FILESIZE)?,
            sid: self.try_get(InlineKFile::SID)?,
        };
        Ok(obj)
    }

    fn to_kfile_meta(self) -> AResult<KFileMeta> {
        Ok(KFileMeta {
            tid: self.try_get(KFileMeta::TID)?,
            omit_tid: self.try_get(KFileMeta::OMIT_TID)?,
            archor: self.try_get(KFileMeta::ARCHOR)?,
            inline: self.try_get(KFileMeta::INLINE)?,
            sid: self.try_get(KFileMeta::SID)?,
            id: self.try_get(KFileMeta::ID)?,
        })
    }
}

impl KDbExecutor<'_> {
    async fn query_kfile_meta_by_id(&self, id: String) -> AResult<Option<KFileMeta>> {
        self.qry_opt(KFileMeta::query_by_pkey_sql(id, OmitTID::never()), |e| {
            e.to_kfile_meta()
        })
        .await
    }
}

impl KFileMapper for KDb {
    async fn ensure_table_kfile(&self) -> EResult {
        self.ensure_table_inline_kfile().await?;
        self.conn()
            .await?
            .exec(KFile::create_sql())
            .await
            .map(|_| ())
    }

    async fn ensure_table_inline_kfile(&self) -> EResult {
        self.conn()
            .await?
            .exec(InlineKFile::create_sql())
            .await
            .map(|_| ())
    }

    async fn insert_kfile(&self, meta: KFileMeta, kfile: KFile) -> EResult {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.exec(
            kfile
                .to_sql_inserter()
                .on_conflict(OnConflict::Replace(KFile::SID.to_string())),
        )
        .await?;
        tx.exec(meta.to_sql_inserter()).await?;
        tx.cmt().await?;
        Ok(())
    }

    async fn query_kfile_by_sid(&self, sid: &str) -> AResult<KFile> {
        let conn = self.conn().await?;
        let res = conn
            .qry_one(
                KFile::query_by_pkey_sql(sid.to_string()),
                |e| e.to_kfile(),
                false,
            )
            .await?;
        Ok(res)
    }

    async fn insert_inline_kfile(
        &self,
        mut req: KReq<InsertInlineKFileReq>,
    ) -> anyhow::Result<InsertInlineKFileRsp> {
        let mut bh = blake3::Hasher::new();
        bh.write_all(req.body.res.content.as_bytes())?;
        let sid = bh.finalize().to_string();

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        req.body.res.sid = sid.clone();

        tx.as_executor()
            .exec(
                KFileMeta {
                    tid: TID::default(),
                    omit_tid: OmitTID::never(),
                    archor: false,
                    inline: false,
                    sid: sid.clone(),
                    id: req.meta_id.clone(),
                }
                .to_sql_inserter(),
            )
            .await?;
        tx.exec(
            req.body
                .res
                .to_sql_inserter()
                .on_conflict(OnConflict::Ignore),
        )
        .await?;

        tx.cmt().await?;

        Ok(InsertInlineKFileRsp { true_sid: sid })
    }

    async fn query_inline_kfile(
        &self,
        req: KReq<QueryInlineKFileReq>,
    ) -> anyhow::Result<QueryInlineKFileRsp> {
        let sid = if let Some(key) = req.meta_id.clone() {
            let sid = self
                .conn()
                .await?
                .as_executor()
                .query_kfile_meta_by_id(key)
                .await?
                .map(|e| e.tid);
            sid
        } else {
            None
        };

        let query = SqlBuilder::read_all(InlineKFile::TABLE)
            .r#where(Wheres::and([
                Wheres::if_some(req.content_type.to_owned(), |e| {
                    Wheres::equal(InlineKFile::CONTENT_TYPE, e)
                }),
                Wheres::if_some(req.sid.to_owned(), |e| Wheres::equal(InlineKFile::SID, e)),
                Wheres::if_some(req.name_like.to_owned(), |e| {
                    Wheres::ilike(InlineKFile::NAME, e, chin_sql::ILikeType::Fuzzy)
                }),
                Wheres::if_some(sid, |e| Wheres::equal(InlineKFile::SID, e)),
            ]))
            .sov("order by tid desc")
            .custom(LimitOffset::new(1));

        let res = self
            .conn()
            .await?
            .qry_list(query, |t| t.to_inline_kfile())
            .await?;

        Ok(QueryInlineKFileRsp { res })
    }
}
