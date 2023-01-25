use axum::{routing::get, Router, Form};

struct Login {
    user_name: String,
    password: String
}

async fn login(form: Form<Login>) {
    
}

async fn hello() -> &'static str {
    "Hello, World!"
}

pub fn app() -> Router {
    Router::new().route("/", get(hello))
}
