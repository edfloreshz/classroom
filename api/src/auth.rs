use crate::{
    prelude::*,
    services::user::{self, User},
};
use axum::{
    body::Body,
    extract::Request,
    http::{self, Response},
    middleware::Next,
    response::IntoResponse,
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

#[derive(Debug)]
pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let jwt_token: String = "randomstring".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Cliams { iat, exp, email };
    let secret = jwt_token.clone();

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>, StatusCode> {
    let secret = "randomstring".to_string();

    let result: Result<TokenData<Cliams>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn authorize(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            })
        }
    };

    let mut header = auth_header.split_whitespace();

    let (_bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    };

    let current_user = match user::get_user_by_email(&state.pool, token_data.claims.email).await {
        Ok(user) => user,
        Err(_) => {
            return Err(AuthError {
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<Value>, StatusCode> {
    let user = match user::get_user_by_email(&state.pool, user_data.email).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&user_data.password, &user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "token": token
    })))
}

pub async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<RegisterData>,
) -> Result<StatusCode, ServerError> {
    if let Err(report) = user_data.validate() {
        return Err(ServerError::new(report, StatusCode::BAD_REQUEST));
    }

    let exists = user::user_exists(&state.pool, &user_data.email)
        .await
        .map_err(ServerError::internal_server_error)?;
    if exists {
        return Err(ServerError::new(
            "User already exists",
            StatusCode::CONFLICT,
        ));
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

    Ok(StatusCode::OK)
}
