use axum::{Json, Router, routing::post};

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::toent::logic::{timeevent::TimeEvent, todoevent::TodoEvent},
};

use super::*;

async fn toent_time_event_guess(
    Json(req): Json<ToentGuessReq>,
) -> KResponse<ToentGuessRsp<TimeEvent>> {
    let words: Words<'_> = req.input.as_str().into();
    let rest: Option<Vec<GuessElem<TimeEvent>>> = TimeEvent::guess(&words);
    let rsp = ToentGuessRsp {
        toents: rest
            .unwrap_or_default()
            .into_iter()
            .map(|e| e.toent)
            .collect(),
    };

    Ok(rsp).into()
}

async fn toent_todo_event_guess(
    Json(req): Json<ToentGuessReq>,
) -> KResponse<ToentGuessRsp<TodoEvent>> {
    let words: Words<'_> = req.input.as_str().into();
    let rest: Option<Vec<GuessElem<TodoEvent>>> = TodoEvent::guess(&words);
    let rsp = ToentGuessRsp {
        toents: rest
            .unwrap_or_default()
            .into_iter()
            .map(|e| e.toent)
            .collect(),
    };

    Ok(rsp).into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(
            "/api/v1/toent-timeevent-guess",
            post(toent_time_event_guess),
        )
        .route(
            "/api/v1/toent-todoevent-guess",
            post(toent_todo_event_guess),
        )
}
