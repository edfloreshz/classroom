use axum::{routing::get, Router};
use users::users;

use crate::state::AppState;
mod users;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/users", get(users))
        .with_state(state)
}

pub async fn index() -> String {
    "Welcome to Classroom!".into()
}
