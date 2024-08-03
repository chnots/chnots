use chin_tools::wrapper::anyhow::AResult;
use serde::Serialize;
use serde_json::json;

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE,
        },
        HeaderValue,
    },
    response::IntoResponse,
    Json, Router,
};

use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::{self, TraceLayer},
};
use tracing::{info, Level};

use crate::app::ShareAppState;

pub mod v1;

pub struct KResponse<E: Serialize>(AResult<E>);

impl<E: Serialize> From<AResult<E>> for KResponse<E> {
    fn from(value: AResult<E>) -> Self {
        Self(value)
    }
}

impl<E: Serialize> IntoResponse for KResponse<E> {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            Ok(e) => {
                let mut res = Json(e).into_response();
                *res.status_mut() = StatusCode::OK;
                res
            }
            Err(err) => {
                let mut res = Json(json!({"msg": err.to_string()})).into_response();
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                res
            }
        }
    }
}

pub async fn serve(app_state: ShareAppState) {
    let port = app_state.config.server.as_ref().map_or(3301, |e| e.port);
    let cors_layer = CorsLayer::new()
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods(Any)
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
        .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
        .on_request(|req: &axum::http::Request<axum::body::Body>, _: &_| {
            info!("request: {:?}", req);
        });

    let app = Router::new()
        .merge(v1::routes())
        .with_state(app_state)
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_>::overriding(
            ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        ))
        .layer(DefaultBodyLimit::disable())
        // https://stackoverflow.com/questions/73498537/axum-router-rejecting-cors-options-preflight-with-405-even-with-corslayer/
        .layer(cors_layer)
        .layer(trace_layer);

    let server_url = format!("{}:{}", "0.0.0.0", port);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    info!("server: {}", server_url);

    axum::serve(listener, app).await.unwrap();
}
