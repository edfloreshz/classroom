use classroom_api::error::Error;
use classroom_api::routes::routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let routes = routes();
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, routes).await?;
    Ok(())
}
