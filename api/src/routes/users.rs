use crate::{internal_server_error, state::AppState, InternalServerError};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
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
) -> Result<Json<Vec<User>>, (StatusCode, Json<InternalServerError>)> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .map(Json)
        .map_err(internal_server_error)
}

#[derive(Debug, Deserialize)]
pub struct UserParams {
    user_id: String,
}

pub async fn get_user(
    State(state): State<AppState>,
    Query(params): Query<UserParams>,
) -> Result<Json<User>, (StatusCode, Json<InternalServerError>)> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(params.user_id)
        .fetch_one(&state.pool)
        .await
        .map(Json)
        .map_err(internal_server_error)
}
