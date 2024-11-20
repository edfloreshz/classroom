use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    id: String,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .map(Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
