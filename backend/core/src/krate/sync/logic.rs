use chin_tools::AResult;

use crate::{
    app::ShareAppState,
    krate::sync::dto::{SyncShakeReq, SyncShakeRsp, SyncShakeRspEnum},
    magics::APP_VERSION,
};

async fn sync_shake(state: ShareAppState, req: SyncShakeReq) -> AResult<SyncShakeRsp> {
    let server_id = state.get_instance_id().await?;

    if req.client_app_version != APP_VERSION {
        return Ok(SyncShakeRsp {
            data: SyncShakeRspEnum::NotSameVersion(req.client_app_version, APP_VERSION.to_string()),
        });
    }

    todo!()
}
