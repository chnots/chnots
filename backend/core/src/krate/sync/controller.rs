use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::sync::dto::{SyncShakeReq, SyncShakeRsp, SyncShakeRspEnum},
    magics::APP_VERSION,
};

async fn fetch_same_keys(state: ShareAppState, req: SyncShakeReq) -> KResponse<SyncShakeRsp> {
    todo!()
}
