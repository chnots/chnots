use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::RustEmbed;
use tracing::info;

use crate::app::ShareAppState;

pub fn routes() -> Router<ShareAppState> {
    // Define our app routes, including a fallback option for anything not matched.
    Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/assets/*file", get(static_handler))
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

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        info!("get {:?}", path.as_str());

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
