use std::borrow::Cow;

use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;
use tracing::{debug, info};

use crate::app::ShareAppState;

pub fn routes() -> Router<ShareAppState> {
    // Define our app routes, including a fallback option for anything not matched.
    Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/static/{*file}", get(static_handler))
        .route("/chnots.svg", get(static_handler))
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

// Finally, we use a fallback route for anything that didn't match.
async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

#[derive(RustEmbed)]
#[folder = "../web-dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

pub enum ContentEnum {
    Cow(Cow<'static, [u8]>),
    String(String),
}

pub fn asset_to_response<T: AsRef<str>>(data: Option<(T, ContentEnum)>) -> Response {
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
        debug!("asset {:?}", path.as_str());

        let data = Asset::get(path.as_str()).and_then(|ef| {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Some((mime, ContentEnum::Cow(ef.data)))
        });

        asset_to_response(data)
    }
}
