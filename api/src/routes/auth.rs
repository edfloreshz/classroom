use crate::{
    models::auth::{ActivationParams, RegisterParams, SignInParams},
    prelude::*,
    services,
};

pub async fn activate(
    State(state): State<AppState>,
    Json(params): Json<ActivationParams>,
) -> Result<Json<Value>, ServerError> {
    services::auth::activate(&state.pool, params).await?;
    Ok(Json(json!({
        "message": "User activated"
    })))
}

pub async fn register(
    State(state): State<AppState>,
    Json(params): Json<RegisterParams>,
) -> Result<Json<Value>, ServerError> {
    services::auth::register(&state.pool, params).await?;
    Ok(Json(json!({
        "message": "User created"
    })))
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(params): Json<SignInParams>,
) -> Result<Json<Value>, ServerError> {
    let token = services::auth::sign_in(&state.pool, params).await?;
    Ok(Json(json!({
        "token": token
    })))
}
