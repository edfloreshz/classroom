use thiserror::Error;

use crate::prelude::ServerError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("SQLx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("EnvVar: {0}")]
    Var(#[from] std::env::VarError),
    #[error("Migration: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Bcrypt: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
    #[error("Jwt: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("API: {0}")]
    Api(#[from] ApiError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Password does not match our records")]
    IncorrectPassword,
    #[error("Validation errors: {0}")]
    Validation(garde::Report),
    #[error("User not found: {0}")]
    UserNotFound(sqlx::Error),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("You are not an authorized user")]
    UnauthorizedUser,
    #[error("You are not authorized to access this resource")]
    UnauthorizedRole,
    #[error("Your account is not active, request activation to your admin")]
    InactiveAccount,
}

impl From<Error> for ServerError {
    fn from(error: Error) -> Self {
        match error {
            Error::Api(api_error) => match api_error {
                ApiError::Validation(report) => ServerError::bad_request(&report),
                ApiError::UserNotFound(error) => match error {
                    sqlx::Error::RowNotFound => ServerError::unauthorized(&error.to_string()),
                    _ => ServerError::internal_server_error(error),
                },
                ApiError::IncorrectPassword => ServerError::unauthorized(&api_error.to_string()),
                ApiError::UserAlreadyExists => ServerError::conflict(&api_error),
                ApiError::InactiveAccount => ServerError::forbidden(&api_error),
                ApiError::UnauthorizedUser => ServerError::forbidden(&api_error),
                ApiError::UnauthorizedRole => ServerError::forbidden(&api_error),
            },
            _ => ServerError::internal_server_error(error),
        }
    }
}
