use anyhow::Ok;
use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult};

use crate::{
    MapperType, expand_mt_branch,
    krate::kspace::dto::{KSpaceArchiveReq, KSpaceArchiveRsp},
    magics::NO_KSPACE,
    model::dto::KReq,
};

use super::{
    dto::{KSpaceCommitReq, KSpaceCommitRsp, KSpaceListReq, KSpaceListRsp},
    *,
};

pub trait KSpaceMapper {
    async fn kspace_list(&self, kreq: KReq<KSpaceListReq>) -> AResult<KSpaceListRsp>;
    async fn kspace_commit(&self, kspace: KReq<KSpaceCommitReq>) -> AResult<KSpaceCommitRsp>;

    async fn kspace_archive(&self, kspace: KReq<KSpaceArchiveReq>) -> AResult<KSpaceArchiveRsp>;

    async fn kspace_ensure_table(&self) -> EResult;
    async fn kspace_ensure_data(&self) -> EResult {
        self.kspace_ensure_table().await?;
        let kreq = KReq {
            body: KSpaceListReq {},
            kspace: NO_KSPACE.try_into()?,
            mkspaces: vec![],
        };
        let all_kspaces = self.kspace_list(kreq.clone()).await?.kspaces;
        for data in [
            ("private", "#aa0000", vec!["public", "work"]),
            ("work", "#aa0000", vec!["public"]),
            ("public", "#aa0000", vec![]),
        ] {
            if !all_kspaces.iter().any(|k| k.name.as_str() == data.0) {
                self.kspace_commit(kreq.frame(KSpaceCommitReq {
                    kspace: KSpace {
                        name: data.0.try_into()?,
                        color: data.1.try_into()?,
                        managers: data.2.iter().map(|s| s.to_string()).collect(),
                        tid: TID::now(),
                        public_access: false,
                    },
                }))
                .await?;
            }
        }

        Ok(())
    }
}

impl KSpaceMapper for MapperType {
    async fn kspace_list(&self, req: KReq<KSpaceListReq>) -> AResult<KSpaceListRsp> {
        expand_mt_branch!(self.kspace_list(req))
    }

    async fn kspace_commit(&self, req: KReq<KSpaceCommitReq>) -> AResult<KSpaceCommitRsp> {
        expand_mt_branch!(self.kspace_commit(req))
    }

    async fn kspace_ensure_table(&self) -> EResult {
        expand_mt_branch!(self.kspace_ensure_table())
    }

    async fn kspace_archive(&self, kspace: KReq<KSpaceArchiveReq>) -> AResult<KSpaceArchiveRsp> {
        expand_mt_branch!(self.kspace_archive(kspace))
    }
}
