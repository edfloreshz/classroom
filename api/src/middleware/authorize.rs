use axum::{
    body::Body,
    extract::{Request, State},
    http::{self, Response},
    middleware::Next,
};

use crate::{
    prelude::*,
    services::{auth::decode_jwt, entities::user::Role, user},
};

pub async fn admin(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, ServerError> {
    authorize(state, req, next, Role::Admin).await
}

pub async fn teacher(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, ServerError> {
    authorize(state, req, next, Role::Teacher).await
}

pub async fn student(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, ServerError> {
    authorize(state, req, next, Role::Student).await
}

async fn authorize(
    state: AppState,
    mut req: Request,
    next: Next,
    role: Role,
) -> Result<Response<Body>, ServerError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| ServerError::forbidden(&"Empty header is not allowed"))?,
        None => {
            return Err(ServerError::forbidden(
                &"Please add the JWT token to the header",
            ))
        }
    };

    let mut header = auth_header.split_whitespace();

    let (_bearer, token) = (header.next(), header.next());

    let token_data = decode_jwt(token.unwrap().to_string())?;

    tracing::info!("{:?} {:?}", token_data.claims.role, role);

    if token_data.claims.role.index() > role.index() {
        return Err(ServerError::forbidden(
            &"You are not authorized to access this resource",
        ));
    }

    let current_user = match user::get_user_by_email(&state.pool, token_data.claims.email).await {
        Ok(user) => user,
        Err(_) => return Err(ServerError::unauthorized(&"You are not an authorized user")),
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
