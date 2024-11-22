use std::env;

use crate::{
    prelude::*,
    services::user::{self, User},
};
use axum::{
    body::Body,
    extract::Request,
    http::{self, Response},
    middleware::Next,
};
use chrono::Duration;
use garde::Validate;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct SignInData {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterData {
    #[garde(skip)]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub password: String,
    #[garde(skip)]
    pub first_name: String,
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String) -> Result<String, ServerError> {
    let secret = env::var("SECRET").map_err(ServerError::internal_server_error)?;

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Cliams { iat, exp, email };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(ServerError::internal_server_error)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>, ServerError> {
    let secret = env::var("SECRET").map_err(ServerError::internal_server_error)?;

    let result = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(ServerError::internal_server_error);
    result
}

#[axum::debug_middleware]
pub async fn authorize(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, ServerError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| ServerError::forbidden("Empty header is not allowed"))?,
        None => {
            return Err(ServerError::forbidden(
                "Please add the JWT token to the header",
            ))
        }
    };

    let mut header = auth_header.split_whitespace();

    let (_bearer, token) = (header.next(), header.next());

    let token_data = decode_jwt(token.unwrap().to_string())?;

    let current_user = match user::get_user_by_email(&state.pool, token_data.claims.email).await {
        Ok(user) => user,
        Err(_) => return Err(ServerError::unauthorized("You are not an authorized user")),
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<Value>, ServerError> {
    let user = match user::get_user_by_email(&state.pool, user_data.email).await {
        Ok(user) => user,
        Err(err) => return Err(ServerError::unauthorized(err.to_string())),
    };

    if !verify_password(&user_data.password, &user.password)
        .map_err(ServerError::internal_server_error)?
    {
        return Err(ServerError::unauthorized(
            "Password does not match our records",
        ));
    }

    let token = encode_jwt(user.email)?;

    Ok(Json(json!({
        "token": token
    })))
}

pub async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<RegisterData>,
) -> Result<Json<Value>, ServerError> {
    if let Err(report) = user_data.validate() {
        return Err(ServerError::bad_request(report));
    }

    let exists = user::user_exists(&state.pool, &user_data.email)
        .await
        .map_err(ServerError::internal_server_error)?;
    if exists {
        return Err(ServerError::conflict("User already exists"));
    }

    let user = User {
        id: Uuid::new_v4().to_string(),
        username: user_data.username,
        email: user_data.email,
        password: hash_password(&user_data.password).map_err(ServerError::internal_server_error)?,
        first_name: user_data.first_name,
        ..Default::default()
    };

    user::post(&state.pool, user)
        .await
        .map_err(ServerError::internal_server_error)?;

    Ok(Json(json!({
        "message": "User created"
    })))
}
