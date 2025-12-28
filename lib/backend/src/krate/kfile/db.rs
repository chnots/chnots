use std::io::Write;

use super::{mapper::KFileMapper, *};
use crate::{
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDbConnBehaiver, KDbExecutor, KDbRow, KDbTransactionBehaiver,
            helper::{Ddls, create_tables},
        },
    },
    model::dto::KReq,
};
use chin_tools::EResult;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier};

use chin_sql::{
    LimitOffset, OnConflict, OrderBy, SqlBuilder, SqlField, SqlReader, SqlReaderBuilder, Wheres, str_type::Varchar, time_type::TID
};

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
            otid: value.try_get(KFileMeta::OTID)?,
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

impl KDbExecutor<'_> {
    async fn insert_kfile_meta(&self, mut meta: KFileMeta) -> EResult {
        let old_meta = self
            .query_kfile_meta(KfileMetaFetchReq {
                req_id: KfileMetaFetchReqId::Otid(meta.otid),
                history_and_archor: true.into(),
            })
            .await?
            .meta;
        match old_meta {
            Some(old_meta) => {
                let diff = meta.tid.as_num() - old_meta.tid.as_num();
                if diff > 3_600 * 1_000_000
                /*1 hour */
                {
                    meta.archor = true;
                }
            }
            None => {
                meta.archor = true;
            }
        }
        self.omit_rows::<KFileMeta>(meta.pkey()).await?;
        self.exec(meta.to_sql_inserter()).await?;

        Ok(())
    }

    async fn query_kfile_meta(&self, req: KfileMetaFetchReq) -> anyhow::Result<KfileMetaFetchRsp> {
        let sb: SqlBuilder = if req.history_and_archor.unwrap_or(false) {
            SqlReader::builder(
                [SqlField {
                    alias: None,
                    table_alias: "kfm",
                    field_name: "*",
                }],
                chin_sql::Froms::Table {
                    table_name: KFileMeta::HIST_TABLE,
                    alias: "kfm",
                },
            )
            .wheres(Wheres::and([
                Wheres::equal(KFileMeta::ARCHOR, true),
                match req.req_id {
                    KfileMetaFetchReqId::Otid(tid) => KFileMeta::pkey_cond(tid),
                    KfileMetaFetchReqId::Id(id) => KFileMeta::unikey_id_cond(id),
                },
            ]))
                .order_by([OrderBy::Desc(KFileMeta::TID.into())])
                .limit(LimitOffset::new(1))
            .build()
            .into()
        } else {
            match req.req_id {
                KfileMetaFetchReqId::Otid(tid) => KFileMeta::pkey_reader(tid),
                KfileMetaFetchReqId::Id(id) => KFileMeta::unikey_id_reader(id),
            }
        };
        let meta = self.qry_opt(sb, |e| (&e).try_into()).await?;
        Ok(KfileMetaFetchRsp { meta })
    }
}

impl KFileMapper for KDb {
    async fn ensure_table_kfile(&self) -> EResult {
        create_tables(
            Ddls::new()
                .with_ddl(InlineKFile::create_sql().to_owned_sql())
                .with_ddls(KFileMeta::ddls()),
            self,
        )
        .await?;

        Ok(())
    }

    async fn insert_kfile_meta(&self, meta: KFileMeta) -> EResult {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor().insert_kfile_meta(meta).await?;
        tx.cmt().await
    }

    async fn insert_inline_kfile(
        &self,
        mut req: KReq<KfileInlineUploadReq>,
    ) -> anyhow::Result<KfileInlineUploadRsp> {
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
            otid: req.otid,
        };
        tx.as_executor().insert_kfile_meta(meta).await?;
        tx.exec(
            req.body
                .res
                .to_sql_inserter()
                .on_conflict(OnConflict::Ignore),
        )
        .await?;

        tx.cmt().await?;

        Ok(KfileInlineUploadRsp { true_sid: sid })
    }

    async fn query_inline_kfile_by_sid(
        &self,
        sid: Varchar<100>,
    ) -> anyhow::Result<KfileInlineDownloadBySidRsp> {
        let query = SqlBuilder::read_all(InlineKFile::TABLE)
            .r#where(Wheres::equal(InlineKFile::SID, sid))
            .seg("order by tid desc")
            .custom(LimitOffset::new(1));

        let res = self
            .conn()
            .await?
            .qry_opt(query, |t| (&t).try_into())
            .await?;

        Ok(KfileInlineDownloadBySidRsp { file: res })
    }

    async fn query_inline_kfile(
        &self,
        req: KReq<KfileInlineDownloadReq>,
    ) -> anyhow::Result<KfileInlineDownloadRsp> {
        let meta_rsp = self
            .query_kfile_meta(KfileMetaFetchReq {
                req_id: req.body.req_id,
                history_and_archor: false.into(),
            })
            .await?;
        match meta_rsp.meta {
            Some(meta) => {
                let res = self.query_inline_kfile_by_sid(meta.sid.clone()).await?.file;
                Ok(KfileInlineDownloadRsp {
                    file: res,
                    meta: Some(meta),
                })
            }
            None => Ok(KfileInlineDownloadRsp {
                file: None,
                meta: None,
            }),
        }
    }

    async fn query_kfile_meta(&self, req: KfileMetaFetchReq) -> anyhow::Result<KfileMetaFetchRsp> {
        self.conn().await?.as_executor().query_kfile_meta(req).await
    }

    async fn query_kfile_meta_by_sid(
        &self,
        sid: Varchar<100>,
    ) -> anyhow::Result<KfileMetaFetchRsp> {
        let meta = self
            .conn()
            .await?
            .qry_opt(KFileMeta::unikey_id_reader(sid), |e| (&e).try_into())
            .await?;
        Ok(KfileMetaFetchRsp { meta })
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
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
