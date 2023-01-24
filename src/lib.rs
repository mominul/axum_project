use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, World!"
}

pub fn app() -> Router {
    Router::new().route("/", get(hello))
}
