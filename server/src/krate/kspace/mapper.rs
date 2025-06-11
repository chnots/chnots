use anyhow::Ok;
use chin_tools::{AResult, EResult, SharedStr};

use crate::{
    krate::kkv::{mapper::KKVMapper, KKVOverwriteReq, KKVQueryManyReq, KKVType},
    magics::NO_KSPACE,
    model::dto::KReq,
    MapperType,
};

use super::{
    dto::{KSpaceOverwriteReq, KSpaceOverwriteRsp, KSpaceQueryAllReq, KSpaceQueryAllRsp},
    *,
};

pub(crate) trait KSpaceMapper {
    async fn kspace_read_all(&self, kreq: KReq<KSpaceQueryAllReq>) -> AResult<KSpaceQueryAllRsp>;
    async fn kspace_overwrite(
        &self,
        kspace: KReq<KSpaceOverwriteReq>,
    ) -> AResult<KSpaceOverwriteRsp>;
    async fn kspace_ensure_data(&self) -> EResult {
        let kreq = KReq {
            body: KSpaceQueryAllReq {},
            kspace: NO_KSPACE.to_owned(),
            mkspaces: vec![]
        };
        let all_kspaces = self.kspace_read_all(kreq.clone()).await?.kspaces;
        for data in [
            ("private", "#aa0000", vec!["public", "work"]),
            ("work", "#aa0000", vec!["public"]),
            ("public", "#aa0000", vec![]),
        ] {
            if !all_kspaces.iter().any(|k| k.name == data.0) {
                self.kspace_overwrite(kreq.frame(KSpaceOverwriteReq {
                    kspace: KSpace {
                        name: data.0.to_owned(),
                        color: data.1.to_owned(),
                        managers: data.2.iter().map(|s| s.to_string()).collect(),
                    },
                })).await?;
            }
        }

        Ok(())
    }
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
            value: serde_json::to_string(&req.body.kspace)?,
        }))
        .await?;

        Ok(KSpaceOverwriteRsp {})
    }
}
