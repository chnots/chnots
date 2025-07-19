use chin_tools::AResult;

use crate::{
    app::ShareAppState,
    krate::sync::{
        dto::{SyncFetchAbsentRsp, SyncFetchDataReq, SyncShakeReq, SyncShakeRsp, SyncShakeRspEnum},
        mapper::{Dumper, SyncMapper},
    },
    magics::APP_VERSION, mapper::TheSameKey,
};

impl ShareAppState {
    pub(crate) async fn sync_shake(&self, req: SyncShakeReq) -> AResult<SyncShakeRsp> {
        let instace_id = self.get_instance_id().await?;

        if req.app_version.as_str() != APP_VERSION {
            return Ok(SyncShakeRsp {
                instance_id: instace_id,
                data: SyncShakeRspEnum::NotSameVersion(APP_VERSION.to_owned()),
            });
        }

        let sync_time = self
            .mapper
            .get_sync_time(
                req.table_name.to_string().try_into()?,
                req.client_id.try_into()?,
            )
            .await?;

        Ok(SyncShakeRsp {
            instance_id: instace_id,
            data: SyncShakeRspEnum::BeginSync { sync_time },
        })
    }
}
