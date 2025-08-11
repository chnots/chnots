use anyhow::Ok;
use chin_sql::{SqlBuilder, time_type::TID};

use crate::{
    krate::kspace::{
        KSpace,
        dto::{KSpaceDeletionRsp, KSpaceOverwriteRsp, KSpaceQueryAllRsp},
        mapper::KSpaceMapper,
    },
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow,
            KDbRowBehavier, KDbTransactionBehaiver, helper::create_tables,
        },
    },
};

impl KSpaceMapper for KDb {
    async fn kspace_read_all(
        &self,
        _: crate::model::dto::KReq<super::dto::KSpaceQueryAllReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceQueryAllRsp> {
        let kspaces = self
            .conn()
            .await?
            .qry_list(SqlBuilder::read_all(KSpace::TABLE), |r| (&r).try_into())
            .await?;
        Ok(KSpaceQueryAllRsp { kspaces })
    }

    async fn kspace_overwrite(
        &self,
        kspace: crate::model::dto::KReq<super::dto::KSpaceOverwriteReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceOverwriteRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.as_executor()
            .omit_rows::<KSpace>(KSpace::pkey_cond(
                kspace.body.kspace.name.as_str().to_lowercase().try_into()?,
            ))
            .await?;
        tx.exec(kspace.body.kspace.to_sql_inserter()).await?;
        tx.cmt().await?;

        Ok(KSpaceOverwriteRsp {})
    }

    async fn kspace_ensure_table(&self) -> chin_tools::EResult {
        create_tables(
            vec![KSpace::create_sql().to_owned_sql(), KSpace::hist_table()],
            self,
        )
        .await
    }

    async fn kspace_delete(
        &self,
        kspace: crate::model::dto::KReq<super::dto::KSpaceDeletionReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceDeletionRsp> {
        self.conn()
            .await?
            .as_executor()
            .omit_rows::<KSpace>(KSpace::pkey_cond(
                kspace.body.kspace_name.as_str().to_lowercase().try_into()?,
            ))
            .await?;
        Ok(KSpaceDeletionRsp {})
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
