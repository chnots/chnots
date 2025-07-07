use anyhow::Ok;
use chin_sql::{SqlBuilder, Wheres};

use crate::{
    krate::kspace::{
        KSpace,
        dto::{KSpaceOverwriteRsp, KSpaceQueryAllRsp},
        mapper::KSpaceMapper,
    },
    mapper::db::{
        KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier, helper::create_tables,
    },
    model::omit_tid::OmitTID,
};

impl<'a> TryFrom<&'a KDbRow> for KSpace {
    type Error = anyhow::Error;

    fn try_from(value: &'a KDbRow) -> Result<Self, Self::Error> {
        let r = KSpace {
            name: value.try_get(KSpace::NAME)?,
            omit_tid: value.try_get(KSpace::OMIT_TID)?,
            color: value.try_get(KSpace::COLOR)?,
            managers: {
                let s: String = value.try_get(KSpace::MANAGERS)?;
                serde_json::from_str(s.as_str())?
            },
            tid: value.try_get(KSpace::TID)?,
        };

        Ok(r)
    }
}

impl KSpaceMapper for KDb {
    async fn kspace_read_all(
        &self,
        _: crate::model::dto::KReq<super::dto::KSpaceQueryAllReq>,
    ) -> chin_tools::AResult<super::dto::KSpaceQueryAllRsp> {
        let kspaces = self
            .conn()
            .await?
            .qry_list(
                SqlBuilder::read_all(KSpace::TABLE)
                    .r#where(Wheres::equal(KSpace::OMIT_TID, OmitTID::never())),
                |r| (&r).try_into(),
            )
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
        create_tables(vec![KSpace::create_sql()], self).await
    }
}
