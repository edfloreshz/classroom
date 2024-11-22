mod auth;
mod index;
mod users;

use crate::prelude::*;

pub fn index() -> MethodRouter<AppState> {
    get(index::index)
}

pub fn users(state: &AppState) -> MethodRouter<AppState> {
    get(users::get_all).layer(middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::authorize,
    ))
}

pub fn user(state: &AppState) -> MethodRouter<AppState> {
    get(users::get)
        .post(users::post)
        .delete(users::delete)
        .put(users::put)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::authorize,
        ))
}

pub fn sign_in() -> MethodRouter<AppState> {
    post(auth::sign_in)
}

pub fn register() -> MethodRouter<AppState> {
    post(auth::register)
}
