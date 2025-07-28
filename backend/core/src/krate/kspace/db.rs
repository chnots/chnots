use anyhow::Ok;
use chin_sql::SqlBuilder;

use crate::{
    krate::kspace::{
        KSpace,
        dto::{KSpaceOverwriteRsp, KSpaceQueryAllRsp},
        mapper::KSpaceMapper,
    },
    mapper::db::{
        HistCreateSql, KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
        helper::create_tables,
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
        self.conn()
            .await?
            .exec(kspace.body.kspace.to_sql_inserter())
            .await?;

        Ok(KSpaceOverwriteRsp {})
    }

    async fn kspace_ensure_table(&self) -> chin_tools::EResult {
        create_tables(
            vec![KSpace::create_sql().to_owned_sql(), KSpace::hist_table()],
            self,
        )
        .await
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
        })
    }
}
