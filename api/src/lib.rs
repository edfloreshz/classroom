use axum::{http::StatusCode, Json};
use serde::Serialize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod database;
mod error;
pub mod routes;
mod state;

pub mod prelude {
    pub use crate::database::connection::connection_pool;
    pub use crate::error::Error;
    pub use crate::routes::{index, users};
    pub use crate::state::AppState;
}

use prelude::*;

pub fn trace() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn env() {
    dotenv::dotenv().ok();
}

pub async fn state() -> Result<AppState, Error> {
    let pool = connection_pool().await?;
    Ok(AppState::new(pool))
}

#[derive(Debug, Serialize)]
pub struct InternalServerError {
    error: String,
}

fn internal_server_error(err: sqlx::Error) -> (StatusCode, Json<InternalServerError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(InternalServerError {
            error: err.to_string(),
        }),
    )
}
