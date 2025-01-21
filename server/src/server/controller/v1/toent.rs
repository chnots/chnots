use axum::{routing::post, Json, Router};

use crate::{
    app::ShareAppState,
    model::dto::
        chnot::{ToentGuessReq, ToentGuessRsp}
    ,
    server::controller::KResponse,
    toent::PossibleToent,
};

async fn toent_guess(Json(req): Json<ToentGuessReq>) -> KResponse<ToentGuessRsp> {
    let rest = PossibleToent::guess(req.input.as_str());
    let rsp = ToentGuessRsp { toents: rest };

    Ok(rsp).into()
}

pub fn routes() -> Router<ShareAppState> {
    Router::new().route("/api/v1/toent-guess", post(toent_guess))
}
