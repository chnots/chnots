use std::io::Write;

use super::{mapper::KFileMapper, *};
use crate::{
    krate::kkv::{KKVOverwriteReq, KKVQueryOneReq},
    mapper::db::{KDbConnBehaiver, KDbExecutor, KDbRow, KDbTransactionBehaiver, ToSqlInserter},
    model::{dto::KReq, omit_tid::OmitTID},
    util::result_util::UnwrapOr,
};
use chin_tools::{AResult, EResult};
use tracing::info;

use super::mapper::KFileDeserializeMapper;
use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier};

use chin_sql::{LimitOffset, OnConflict, SqlReader, Wheres};

impl KFileDeserializeMapper for KDbRow {
    fn to_inline_kfile(self) -> AResult<InlineKFile> {
        let obj = InlineKFile {
            tid: self.try_get(InlineKFile::TID)?,
            name: self.try_get(InlineKFile::NAME)?,
            content: self.try_get(InlineKFile::CONTENT)?,
            content_type: self.try_get(InlineKFile::CONTENT_TYPE)?,
            omit_tid: self.try_get(InlineKFile::OMIT_TID)?,
            kspace: self.try_get(InlineKFile::KSPACE)?,
            archor: self.try_get(InlineKFile::ARCHOR)?,
            sid: self.try_get(InlineKFile::SID)?,
        };
        Ok(obj)
    }

    fn to_kfile(self) -> AResult<KFile> {
        let obj = KFile {
            tid: self.try_get(KFile::TID)?,
            omit_tid: self.try_get(KFile::OMIT_TID)?,
            kspace: self.try_get(KFile::KSPACE)?,
            ori_filename: self.try_get(KFile::ORI_FILENAME)?,
            content_type: self.try_get(KFile::CONTENT_TYPE)?,
            ori_last_modified: self.try_get(KFile::ORI_LAST_MODIFIED)?,
            filesize: self.try_get(KFile::FILESIZE)?,
            sid: self.try_get(InlineKFile::SID)?,
        };
        Ok(obj)
    }
}

impl KDbExecutor<'_> {}

impl KFileMapper for KDb {
    async fn ensure_table_kfile(&self) -> EResult {
        self.ensure_table_inline_kfile().await?;
        self.conn()
            .await?
            .create_table(KFile::schema(self.db_type()))
            .await
    }

    async fn ensure_table_inline_kfile(&self) -> EResult {
        self.conn()
            .await?
            .create_table(InlineKFile::schema(self.db_type()))
            .await
    }

    async fn insert_kfile(&self, res: KFile) -> EResult {
        let conn = self.conn().await?;

        conn.exec(
            res.to_sql_inserter()
                .on_conflict(OnConflict::Replace(KFile::SID.to_string())),
        )
        .await?;
        Ok(())
    }

    async fn query_kfile_by_sid(&self, sid: &str) -> AResult<KFile> {
        let conn = self.conn().await?;
        let res = conn
            .qry_one(
                SqlReader::read_all(KFile::TABLE).r#where(Wheres::equal(KFile::SID, sid)),
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
        info!("begin to insert {} -- {}", req.kkv_key, req.res.sid);
        tx.as_executor()
            .kkv_overwrite(req.frame(KKVOverwriteReq {
                key: req.kkv_key.clone(),
                kind: crate::krate::kkv::KKVType::ToKFile,
                value: sid.clone(),
            }))
            .await?;
        req.body.res.sid = sid.clone();
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
        let sid = if let Some(key) = req.kkv_key.clone() {
            let sid = self
                .conn()
                .await?
                .as_executor()
                .kkv_query(req.frame(KKVQueryOneReq {
                    key,
                    kind: crate::krate::kkv::KKVType::ToKFile,
                }))
                .await?
                .value;
            sid
        } else {
            None
        };
        let query = SqlReader::read_all(InlineKFile::TABLE)
            .r#where(Wheres::and([
                Wheres::if_some(req.content_type.to_owned(), |e| {
                    Wheres::equal(InlineKFile::CONTENT_TYPE, e)
                }),
                Wheres::if_some(req.sid.to_owned(), |e| Wheres::equal(InlineKFile::SID, e)),
                Wheres::if_some(req.name_like.to_owned(), |e| {
                    Wheres::ilike(InlineKFile::NAME, e, chin_sql::ILikeType::Fuzzy)
                }),
                Wheres::transform(req.with_del.default_false(), |flag| {
                    if flag {
                        Wheres::None
                    } else {
                        Wheres::equal(InlineKFile::OMIT_TID, OmitTID::never())
                    }
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
