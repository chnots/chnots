use std::borrow::Cow;

use axum::{
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::{Router, get},
};
use rust_embed::RustEmbed;
use log::warn;

use crate::app::ShareAppState;

pub(crate) fn routes() -> Router<ShareAppState> {
    // Define our app routes, including a fallback option for anything not matched.
    Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/static/{*file}", get(static_handler))
        .fallback_service(get(index_handler))
}

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "../../web-dist"]
struct Asset;

pub(crate) struct StaticFile<T>(pub(crate) T);

pub(crate) enum ContentEnum {
    Cow(Cow<'static, [u8]>),
    String(String),
}

pub(crate) fn asset_to_response<T: AsRef<str>>(data: Option<(T, ContentEnum)>) -> Response {
    match data {
        Some((mime, data)) => match data {
            ContentEnum::Cow(data) => {
                ([(header::CONTENT_TYPE, mime.as_ref())], data).into_response()
            }
            ContentEnum::String(data) => {
                ([(header::CONTENT_TYPE, mime.as_ref())], data).into_response()
            }
        },
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        warn!("asset {:?}", path.as_str());

        let data = Asset::get(path.as_str()).map(|ef| {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (mime, ContentEnum::Cow(ef.data))
        });

        asset_to_response(data)
    }
}
