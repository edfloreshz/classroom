use classroom_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    trace();
    env();

    let state = state().await?;

    let routes = axum::Router::new()
        .route("/", routes::index())
        .route("/api/users", routes::users(&state))
        .route("/api/user", routes::user(&state))
        .route("/api/auth/signin", routes::sign_in())
        .route("/api/auth/register", routes::register())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await?;
    Ok(())
}
