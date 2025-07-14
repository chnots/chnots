use std::io::Write;

use super::{mapper::KFileMapper, *};
use crate::{
    mapper::db::{KDbConnBehaiver, KDbRow, KDbTransactionBehaiver},
    model::{
        dto::KReq,
        omit_tid::{OmitNow, OmitTID},
    },
};
use anyhow::Context;
use chin_tools::EResult;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier};

use chin_sql::{LimitOffset, OnConflict, SqlBuilder, Wheres, str_type::Varchar, time_type::TID};

impl TryFrom<KDbRow> for InlineKFile {
    type Error = anyhow::Error;

    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        let obj = InlineKFile {
            tid: value.try_get(InlineKFile::TID)?,
            content: value.try_get(InlineKFile::CONTENT)?,
            sid: value.try_get(InlineKFile::SID)?,
        };
        Ok(obj)
    }
}
impl TryFrom<KDbRow> for KFileMeta {
    type Error = anyhow::Error;
    fn try_from(value: KDbRow) -> Result<Self, Self::Error> {
        Ok(KFileMeta {
            filename: value.try_get(KFileMeta::FILENAME)?,
            tid: value.try_get(KFileMeta::TID)?,
            omit_tid: value.try_get(KFileMeta::OMIT_TID)?,
            archor: value.try_get(KFileMeta::ARCHOR)?,
            inline: value.try_get(KFileMeta::INLINE)?,
            sid: value.try_get(KFileMeta::SID)?,
            id: value.try_get(KFileMeta::ID)?,
            content_type: value.try_get(KFileMeta::CONTENT_TYPE)?,
            last_modified: value.try_get(KFileMeta::LAST_MODIFIED)?,
            filesize: value.try_get(KFileMeta::FILESIZE)?,
        })
    }
}

impl KFileMapper for KDb {
    async fn ensure_table_kfile(&self) -> EResult {
        for sql in KFileMeta::create_sql()
            .sqls(self.get_db_type())?
            .into_iter()
        {
            self.conn().await?.exec(sql).await?;
        }
        for sql in InlineKFile::create_sql()
            .sqls(self.get_db_type())?
            .into_iter()
        {
            self.conn().await?.exec(sql).await?;
        }

        Ok(())
    }

    async fn insert_kfile(&self, meta: KFileMeta) -> EResult {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.exec(
            KFileMeta::pkey_updater(meta.id.clone(), meta.omit_tid).omit_now(KFileMeta::OMIT_TID),
        )
        .await?;
        tx.exec(meta.to_sql_inserter()).await?;
        tx.cmt().await?;

        Ok(())
    }

    async fn insert_inline_kfile(
        &self,
        mut req: KReq<InsertInlineKFileReq>,
    ) -> anyhow::Result<InsertInlineKFileRsp> {
        let mut bh = blake3::Hasher::new();
        let bytes = req.body.res.content.as_str().as_bytes();
        bh.write_all(bytes)?;
        let sid: Varchar<100> = bh.finalize().to_string().try_into()?;

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        req.body.res.sid = sid.clone();

        let meta = KFileMeta {
            tid: TID::default(),
            omit_tid: OmitTID::never(),
            archor: false,
            inline: true,
            sid: sid.clone(),
            id: req.meta_id.clone(),
            filename: req
                .filename
                .clone()
                .unwrap_or(format!("inline-kfile-{}", &TID::default().as_num()).try_into()?),
            content_type: req.content_type.clone(),
            last_modified: TID::default(),
            filesize: bytes.len() as i64,
        };
        tx.exec(
            KFileMeta::pkey_updater(meta.id.clone(), meta.omit_tid).omit_now(KFileMeta::OMIT_TID),
        )
        .await?;
        tx.exec(meta.to_sql_inserter()).await?;
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
                .qry_opt(KFileMeta::pkey_reader(key.clone(), OmitTID::never()), |e| {
                    let c: InlineKFile = e.try_into()?;
                    Ok(c)
                })
                .await?
                .map(|e| e.sid);
            Some(sid.context(format!("unable to find sid for {key}"))?)
        } else {
            None
        };

        let query = SqlBuilder::read_all(InlineKFile::TABLE)
            .r#where(Wheres::and([
                Wheres::if_some(req.sid.to_owned(), |e| Wheres::equal(InlineKFile::SID, e)),
                Wheres::if_some(sid, |e| Wheres::equal(InlineKFile::SID, e)),
            ]))
            .sov("order by tid desc")
            .custom(LimitOffset::new(1));

        let res = self.conn().await?.qry_list(query, |t| t.try_into()).await?;

        Ok(QueryInlineKFileRsp { res })
    }

    async fn query_kfile_meta(&self, req: QueryKFileReq) -> anyhow::Result<QueryKFileMetaRsp> {
        let meta = self
            .conn()
            .await?
            .qry_opt(KFileMeta::pkey_reader(req.meta_id, OmitTID::never()), |e| {
                e.try_into()
            })
            .await?;
        Ok(QueryKFileMetaRsp { meta })
    }

    async fn query_kfile_meta_by_sid(
        &self,
        sid: Varchar<100>,
    ) -> anyhow::Result<QueryKFileMetaRsp> {
        let meta = self
            .conn()
            .await?
            .qry_opt(KFileMeta::pkey_reader(sid, OmitTID::never()), |e| {
                e.try_into()
            })
            .await?;
        Ok(QueryKFileMetaRsp { meta })
    }
}
