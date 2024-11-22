use std::env;

use crate::{
    error::ApiError,
    models::{
        auth::{Cliams, RegisterParams, SignInParams},
        user::UserParams,
    },
    prelude::*,
    services::user,
};
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use uuid::Uuid;

use super::entities::user::Role;

pub async fn sign_in(pool: &Pool<Sqlite>, params: SignInParams) -> Result<String, Error> {
    params.validate().map_err(ApiError::Validation)?;

    let user = user::get_user_by_email(pool, params.email)
        .await
        .map_err(ApiError::UserNotFound)?;

    if !verify_password(&params.password, &user.password)? {
        return Err(ApiError::IncorrectPassword.into());
    }

    let token = encode_jwt(user.email, user.role)?;

    Ok(token)
}

pub async fn register(pool: &Pool<Sqlite>, params: RegisterParams) -> Result<(), Error> {
    params.validate().map_err(ApiError::Validation)?;

    let exists = user::user_exists(pool, &params.email).await?;
    if exists {
        return Err(ApiError::UserAlreadyExists.into());
    }

    let user = UserParams {
        id: Uuid::new_v4().to_string(),
        username: params.username,
        email: params.email,
        password: hash_password(&params.password)?,
        first_name: params.first_name,
        last_name: params.last_name,
        ..Default::default()
    };

    user::post(pool, user).await?;

    Ok(())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String, role: Role) -> Result<String, Error> {
    let secret = env::var("SECRET")?;

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Cliams {
        iat,
        exp,
        email,
        role,
    };

    let encoded = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(encoded)
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
