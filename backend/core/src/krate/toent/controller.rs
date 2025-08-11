use axum::{Json, Router, routing::post};

use crate::{app::ShareAppState, controller::KResponse};

use super::*;

async fn toent_guess(Json(req): Json<ToentGuessReq>) -> KResponse<ToentGuessRsp> {
    let rest = PossibleToent::guess(req.input.as_str());
    let rsp = ToentGuessRsp { toents: rest };

    Ok(rsp).into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new().route("/api/v1/toent-guess", post(toent_guess))
}
