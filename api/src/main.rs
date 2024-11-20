use classroom_api::routes;
use classroom_api::state::AppState;
use classroom_api::{database, Error};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    classroom_api::trace();
    dotenv::dotenv().ok();
    let pool = database::pool().await?;
    let state = AppState::new(pool);
    let routes = routes::routes(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await?;
    Ok(())
}
