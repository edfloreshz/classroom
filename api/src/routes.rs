use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(index))
}

pub async fn index() -> String {
    "Welcome to Classroom!".into()
}
