use anyhow::Ok;
use chin_sql::{SqlBuilder, time_type::TID};

use crate::{
    krate::kspace::{
        KSpace,
        dto::{KSpaceArchiveRsp, KSpaceCommitRsp, KSpaceListRsp},
        mapper::KSpaceMapper,
    },
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow,
            KDbRowBehavier, KDbTransactionBehaiver,
            helper::{Ddls, print_ddls},
        },
    },
};

impl KSpaceMapper for KDb {
    async fn kspace_list(
        &self,
        _: crate::model::dto::KReq<super::dto::KSpaceListReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceListRsp> {
        let kspaces = self
            .conn()
            .await?
            .qry_list(SqlBuilder::read_all(KSpace::TABLE), |r| (&r).try_into())
            .await?;
        Ok(KSpaceListRsp { kspaces })
    }

    async fn kspace_commit(
        &self,
        kspace: crate::model::dto::KReq<super::dto::KSpaceCommitReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows::<KSpace>(KSpace::pkey_cond(
                kspace.body.kspace.name.as_str().to_lowercase().try_into()?,
            ))
            .await?;
        tx.exec(kspace.body.kspace.to_sql_inserter()).await?;
        tx.cmt().await?;

        Ok(KSpaceCommitRsp {})
    }

    async fn kspace_ensure_table(&self) -> chin_tools::EResult {
        print_ddls(Ddls::new().with_ddls(KSpace::ddls()), self).await
    }

    async fn kspace_archive(
        &self,
        kspace: crate::model::dto::KReq<super::dto::KSpaceArchiveReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceArchiveRsp> {
        self.conn()
            .await?
            .as_executor()
            .omit_rows::<KSpace>(KSpace::pkey_cond(
                kspace.body.kspace_name.as_str().to_lowercase().try_into()?,
            ))
            .await?;
        Ok(KSpaceArchiveRsp {})
    }
}

impl TryFrom<&KDbRow> for KSpace {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.try_get(Self::NAME)?,
            color: value.try_get(Self::COLOR)?,
            managers: {
                let s: String = value.try_get(Self::MANAGERS)?;
                serde_json::from_str(s.as_str())?
            },
            tid: value.try_get(Self::TID)?,
            public_access: value.try_get(Self::PUBLIC_ACCESS)?,
        })
    }
}

impl Curd for KSpace {
    fn pkey(&self) -> chin_sql::Wheres<'_> {
        Self::pkey_cond(self.name.clone())
    }
    fn tid(&self) -> TID {
        self.tid
    }
}
