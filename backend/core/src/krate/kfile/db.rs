use std::io::Write;

use super::{mapper::KFileMapper, *};
use crate::{
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDbConnBehaiver, KDbRow, KDbTransactionBehaiver, helper::create_tables,
        },
    },
    model::dto::KReq,
};
use anyhow::Context;
use chin_tools::EResult;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier};

use chin_sql::{LimitOffset, OnConflict, SqlBuilder, Wheres, str_type::Varchar, time_type::TID};

impl TryFrom<&KDbRow> for InlineKFile {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = InlineKFile {
            tid: value.try_get(InlineKFile::TID)?,
            content: value.try_get(InlineKFile::CONTENT)?,
            sid: value.try_get(InlineKFile::SID)?,
        };
        Ok(obj)
    }
}
impl TryFrom<&KDbRow> for KFileMeta {
    type Error = anyhow::Error;
    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(KFileMeta {
            filename: value.try_get(KFileMeta::FILENAME)?,
            tid: value.try_get(KFileMeta::TID)?,
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
        create_tables(
            vec![
                InlineKFile::create_sql().to_owned_sql(),
                KFileMeta::create_sql().to_owned_sql(),
                KFileMeta::hist_table(),
            ],
            self,
        )
        .await?;

        Ok(())
    }

    async fn insert_kfile(&self, meta: KFileMeta) -> EResult {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        tx.as_executor()
            .omit_rows::<KFileMeta>(KFileMeta::pkey_cond(meta.id.clone()))
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
        tx.as_executor()
            .omit_rows::<KFileMeta>(KFileMeta::pkey_cond(meta.id.clone()))
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

    async fn query_inline_kfile_by_sid(
        &self,
        sid: Varchar<100>,
    ) -> anyhow::Result<KFileInlineGetBySidRsp> {
        let query = SqlBuilder::read_all(InlineKFile::TABLE)
            .r#where(Wheres::equal(InlineKFile::SID, sid))
            .sov("order by tid desc")
            .custom(LimitOffset::new(1));

        let res = self
            .conn()
            .await?
            .qry_opt(query, |t| (&t).try_into())
            .await?;

        Ok(KFileInlineGetBySidRsp { file: res })
    }

    async fn query_inline_kfile(
        &self,
        req: KReq<QueryInlineKFileReq>,
    ) -> anyhow::Result<QueryInlineKFileRsp> {
        let sid: Varchar<100> = if let Some(sid) = &req.sid {
            sid.clone()
        } else if let Some(key) = req.meta_id.clone() {
            let sid = self
                .conn()
                .await?
                .as_executor()
                .qry_opt(KFileMeta::pkey_reader(key.clone()), |e| {
                    let c: KFileMeta = (&e).try_into()?;
                    Ok(c)
                })
                .await?
                .map(|e| e.sid);
            sid.context(format!("unable to find sid for {key}"))?
        } else {
            anyhow::bail!("there are no meta_id and sid")
        };

        let res = self.query_inline_kfile_by_sid(sid).await?.file;

        Ok(QueryInlineKFileRsp {
            res: res.map(|e| vec![e]).unwrap_or(vec![]),
        })
    }

    async fn query_kfile_meta(&self, req: QueryKFileReq) -> anyhow::Result<QueryKFileMetaRsp> {
        let meta = self
            .conn()
            .await?
            .qry_opt(KFileMeta::pkey_reader(req.meta_id), |e| (&e).try_into())
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
            .qry_opt(KFileMeta::pkey_reader(sid), |e| (&e).try_into())
            .await?;
        Ok(QueryKFileMetaRsp { meta })
    }

    async fn insert_inline_kfile2(&self, req: InlineKFile) -> chin_tools::AResult<usize> {
        self.conn()
            .await?
            .exec(req.to_sql_inserter().on_conflict(OnConflict::Ignore))
            .await
    }
}

impl Curd for KFileMeta {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.id.clone())
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
