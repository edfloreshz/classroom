use crate::{
    prelude::*,
    services::user::{self, User, UserParams},
};

pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<User>>, ServerError> {
    user::get_all(&state.pool)
        .await
        .map(Json)
        .map_err(ServerError::internal_server_error)
}

pub async fn get(
    State(state): State<AppState>,
    Query(params): Query<UserParams>,
) -> Result<Json<User>, ServerError> {
    user::get(&state.pool, params)
        .await
        .map(Json)
        .map_err(ServerError::internal_server_error)
}

pub async fn post(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<StatusCode, ServerError> {
    user::post(&state.pool, user)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(ServerError::internal_server_error)
}

pub async fn put(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<StatusCode, ServerError> {
    user::put(&state.pool, user)
        .await
        .map(|_| StatusCode::OK)
        .map_err(ServerError::internal_server_error)
}

pub async fn delete(
    State(state): State<AppState>,
    Query(params): Query<UserParams>,
) -> Result<StatusCode, ServerError> {
    user::delete(&state.pool, params)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ServerError::internal_server_error)
}
