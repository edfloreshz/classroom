mod index;
mod users;

use crate::prelude::*;

pub fn index() -> MethodRouter<AppState> {
    get(index::index)
}

pub fn users() -> MethodRouter<AppState> {
    get(users::get_all)
}

pub fn user() -> MethodRouter<AppState> {
    get(users::get)
        .post(users::post)
        .delete(users::delete)
        .put(users::put)
}

pub fn sign_in() -> MethodRouter<AppState> {
    post(auth::sign_in)
}

pub fn register() -> MethodRouter<AppState> {
    post(auth::register)
}
