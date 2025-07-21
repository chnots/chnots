use anyhow::Ok;
use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult};

use crate::{
    MapperType, expand_mt_branch,
    magics::NO_KSPACE,
    model::{dto::KReq, },
};

use super::{
    dto::{KSpaceOverwriteReq, KSpaceOverwriteRsp, KSpaceQueryAllReq, KSpaceQueryAllRsp},
    *,
};

pub trait KSpaceMapper {
    async fn kspace_read_all(&self, kreq: KReq<KSpaceQueryAllReq>) -> AResult<KSpaceQueryAllRsp>;
    async fn kspace_overwrite(
        &self,
        kspace: KReq<KSpaceOverwriteReq>,
    ) -> AResult<KSpaceOverwriteRsp>;

    async fn kspace_ensure_table(&self) -> EResult;
    async fn kspace_ensure_data(&self) -> EResult {
        self.kspace_ensure_table().await?;
        let kreq = KReq {
            body: KSpaceQueryAllReq {},
            kspace: NO_KSPACE.try_into()?,
            mkspaces: vec![],
        };
        let all_kspaces = self.kspace_read_all(kreq.clone()).await?.kspaces;
        for data in [
            ("private", "#aa0000", vec!["public", "work"]),
            ("work", "#aa0000", vec!["public"]),
            ("public", "#aa0000", vec![]),
        ] {
            if !all_kspaces.iter().any(|k| k.name.as_str() == data.0) {
                self.kspace_overwrite(kreq.frame(KSpaceOverwriteReq {
                    kspace: KSpace {
                        name: data.0.try_into()?,
                        color: data.1.try_into()?,
                        managers: data.2.iter().map(|s| s.to_string()).collect(),
                        tid: TID::default(),
                    },
                }))
                .await?;
            }
        }

        Ok(())
    }
}

impl KSpaceMapper for MapperType {
    async fn kspace_read_all(&self, req: KReq<KSpaceQueryAllReq>) -> AResult<KSpaceQueryAllRsp> {
        expand_mt_branch!(self.kspace_read_all(req))
    }

    async fn kspace_overwrite(&self, req: KReq<KSpaceOverwriteReq>) -> AResult<KSpaceOverwriteRsp> {
        expand_mt_branch!(self.kspace_overwrite(req))
    }

    async fn kspace_ensure_table(&self) -> EResult {
        expand_mt_branch!(self.kspace_ensure_table())
    }
}
