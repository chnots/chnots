use std::{net::SocketAddr, path::PathBuf};

use axum_server::tls_rustls::RustlsConfig;
use chin_tools::wrapper::anyhow::{AResult, EResult};
use serde::Serialize;
use serde_json::json;

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    http::{
        header::{
            ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
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
use v1::{chnot, llmchat, resource, toent};

use crate::app::ShareAppState;

mod asset;
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
                tracing::error!("Error Occured: {}", err.to_string());
                let mut res = Json(
                    json!({"msg": err.to_string(), "backstrace": err.backtrace().to_string()}),
                )
                .into_response();
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                res
            }
        }
    }
}

pub async fn serve(app_state: ShareAppState) -> EResult {
    let port = app_state.config.server.as_ref().map_or(3301, |e| e.port);
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::DEBUG))
        .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
        .on_request(|_req: &_, _: &_| {});

    let app = Router::new()
        .merge(resource::routes())
        .merge(chnot::routes())
        .merge(asset::routes())
        .merge(toent::routes())
        .merge(llmchat::routes())
        .with_state(app_state.clone())
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

    if let Some(config) = &app_state.config.server {
        let tls_config = RustlsConfig::from_pem_file(
            PathBuf::from(config.tls_cert.clone()),
            PathBuf::from(config.tls_key.clone()),
        )
        .await?;

        axum_server::bind_rustls(
            SocketAddr::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
                port,
            ),
            tls_config,
        )
        .serve(app.into_make_service())
        .await?;
    } else {
        let server_url = format!("{}:{}", "0.0.0.0", port);

        let listener = tokio::net::TcpListener::bind(&server_url).await?;
        info!("server: {}", server_url);

        axum::serve(listener, app).await?;
    }
    Ok(())
}
