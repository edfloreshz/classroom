use crate::{internal_server_error, state::AppState, InternalServerError};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::query;

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

#[derive(Debug, Deserialize)]
pub struct UserParams {
    id: String,
}

pub async fn get_all(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<InternalServerError>)> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
        .map(Json)
        .map_err(internal_server_error)
}

pub async fn get(
    State(state): State<AppState>,
    Query(params): Query<UserParams>,
) -> Result<Json<User>, (StatusCode, Json<InternalServerError>)> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(params.id)
        .fetch_one(&state.pool)
        .await
        .map(Json)
        .map_err(internal_server_error)
}

pub async fn post(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<StatusCode, (StatusCode, Json<InternalServerError>)> {
    query!(
        "INSERT INTO users
        (id, username, email, password, first_name, last_name)
        VALUES
        (?, ?, ?, ?, ?, ?)",
        user.id,
        user.username,
        user.email,
        user.password,
        user.first_name,
        user.last_name,
    )
    .execute(&state.pool)
    .await
    .map_err(internal_server_error)?;
    Ok(StatusCode::CREATED)
}

pub async fn put(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<StatusCode, (StatusCode, Json<InternalServerError>)> {
    sqlx::query!(
        "UPDATE users
        SET username = ?, email = ?, password = ?, first_name = ?, last_name = ?
        WHERE id = ?",
        user.username,
        user.email,
        user.password,
        user.first_name,
        user.last_name,
        user.id,
    )
    .execute(&state.pool)
    .await
    .map_err(internal_server_error)?;
    Ok(StatusCode::OK)
}

pub async fn delete(
    State(state): State<AppState>,
    Query(params): Query<UserParams>,
) -> Result<StatusCode, (StatusCode, Json<InternalServerError>)> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(params.id)
        .execute(&state.pool)
        .await
        .map_err(internal_server_error)?;
    Ok(StatusCode::NO_CONTENT)
}
