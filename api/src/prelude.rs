pub use crate::database::connection_pool;
pub use crate::error::Error;
pub use crate::routes;
pub use crate::state::AppState;
use axum::{body::Body, http::Response, response::IntoResponse};
pub use axum::{
    extract::{Query, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, patch, post, put, MethodRouter},
    Json,
};
pub use chrono::{DateTime, Utc};
pub use garde::Validate;
pub use serde::{Deserialize, Serialize};
pub use serde_json::json;
pub use serde_json::Value;
pub use sqlx::{query, Pool, Sqlite};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

pub struct ServerError {
    message: String,
    status: StatusCode,
}

impl ServerError {
    pub fn new(message: &impl ToString, status: StatusCode) -> Self {
        Self {
            message: message.to_string().clone(),
            status,
        }
    }

    pub fn forbidden(message: &impl ToString) -> Self {
        Self::new(message, StatusCode::FORBIDDEN)
    }

    pub fn bad_request(message: &impl ToString) -> Self {
        Self::new(message, StatusCode::BAD_REQUEST)
    }

    pub fn conflict(message: &impl ToString) -> Self {
        Self::new(message, StatusCode::CONFLICT)
    }

    pub fn unauthorized(message: &impl ToString) -> Self {
        Self::new(message, StatusCode::UNAUTHORIZED)
    }

    pub fn internal_server_error(error: impl Into<Error>) -> Self {
        let error: Error = error.into();
        Self {
            message: error.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status, body).into_response()
    }
}
