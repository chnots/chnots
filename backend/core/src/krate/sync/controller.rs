use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::sync::dto::{SyncShakeReq, SyncShakeRsp},
};

async fn sync_shake(state: ShareAppState, req: SyncShakeReq) -> KResponse<SyncShakeRsp> {
    state.sync_shake(req).await.into()
}
