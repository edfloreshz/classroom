pub mod index;
pub mod users;

use crate::AppState;
use axum::routing::{get, MethodRouter};

pub fn users() -> MethodRouter<AppState> {
    get(users::get_all)
}

pub fn user() -> MethodRouter<AppState> {
    get(users::get)
        .post(users::post)
        .delete(users::delete)
        .put(users::put)
}
