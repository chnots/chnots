use anyhow::Ok;
use chin_tools::{AResult, SharedStr};

use crate::{
    krate::kkv::{mapper::KKVMapper, KKVOverwriteReq, KKVQueryManyReq, KKVType},
    magics::NO_KSPACE,
    model::dto::KReq,
    MapperType,
};

use super::{
    dto::{KSpaceOverwriteRsp, KSpaceOverwriteReq, KSpaceQueryAllReq, KSpaceQueryAllRsp},
    *,
};

pub(crate) trait KSpaceMapper {
    async fn kspace_read_all(&self, kreq: KReq<KSpaceQueryAllReq>) -> AResult<KSpaceQueryAllRsp>;
    async fn kspace_overwrite(
        &self,
        kspace: KReq<KSpaceOverwriteReq>,
    ) -> AResult<KSpaceOverwriteRsp>;
}

impl KSpaceMapper for MapperType {
    async fn kspace_read_all(&self, _: KReq<KSpaceQueryAllReq>) -> AResult<KSpaceQueryAllRsp> {
        let kkvs = self
            .kkv_query_many(KKVQueryManyReq {
                key: None,
                kind: Some(KKVType::KSpaceInfo),
                kspace: Some(SharedStr::new(NO_KSPACE)),
            })
            .await?;

        let kspaces: Result<Vec<KSpace>, serde_json::Error> = kkvs
            .kkvs
            .into_iter()
            .map(|kkv| serde_json::from_str(&kkv.value))
            .collect();

        Ok(KSpaceQueryAllRsp { kspaces: kspaces? })
    }

    async fn kspace_overwrite(&self, req: KReq<KSpaceOverwriteReq>) -> AResult<KSpaceOverwriteRsp> {
        self.kkv_overwrite(req.frame(KKVOverwriteReq {
            key: req.body.kspace.name.clone(),
            kind: KKVType::KSpaceInfo,
            value: serde_json::to_string(&req.body)?,
        }))
        .await?;

        Ok(KSpaceOverwriteRsp {})
    }
}
