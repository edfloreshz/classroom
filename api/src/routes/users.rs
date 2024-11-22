use crate::{
    models::user::{GetUserParams, PostUserParams, PutUserParams, User},
    prelude::*,
    services::user,
};

pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<User>>, ServerError> {
    user::get_all(&state.pool)
        .await
        .map(User::from_vec)
        .map(Json)
        .map_err(ServerError::internal_server_error)
}

pub async fn get(
    State(state): State<AppState>,
    Query(params): Query<GetUserParams>,
) -> Result<Json<User>, ServerError> {
    user::get(&state.pool, params)
        .await
        .map(User::from)
        .map(Json)
        .map_err(ServerError::internal_server_error)
}

pub async fn post(
    State(state): State<AppState>,
    Json(user): Json<PostUserParams>,
) -> Result<StatusCode, ServerError> {
    user::post(&state.pool, user)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(ServerError::internal_server_error)
}

pub async fn put(
    State(state): State<AppState>,
    Json(params): Json<PutUserParams>,
) -> Result<StatusCode, ServerError> {
    user::put(&state.pool, params)
        .await
        .map(|_| StatusCode::OK)
        .map_err(ServerError::internal_server_error)
}

pub async fn delete(
    State(state): State<AppState>,
    Query(params): Query<GetUserParams>,
) -> Result<StatusCode, ServerError> {
    user::delete(&state.pool, params)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(ServerError::internal_server_error)
}
