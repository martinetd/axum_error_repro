use axum::{
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    Router,
};

struct ErrorPage {
    err: anyhow::Error,
}

impl From<anyhow::Error> for ErrorPage {
    fn from(source: anyhow::Error) -> Self {
        ErrorPage { err: source }
    }
}

impl IntoResponse for ErrorPage {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Got error: {:?}\n", self.err)).into_response()
    }
}

async fn slash_get_handler() -> Result<Response, ErrorPage> {
    Err(anyhow::Error::msg("test"))?
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(slash_get_handler));

    axum::Server::bind(&"[::]:12345".parse().expect("Could not parse socket address")).serve(app.into_make_service()).await.unwrap();
}
