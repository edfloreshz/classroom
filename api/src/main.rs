use axum::{routing::get, Router};
use classroom_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    classroom_api::trace();
    classroom_api::env();

    let state = classroom_api::state().await?;
    let routes = Router::new()
        .route("/", get(index::get_index))
        .route("/api/users", get(users::get_users))
        .with_state(state.clone())
        .route("/api/user", get(users::get_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await?;
    Ok(())
}
