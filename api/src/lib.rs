use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod database;
mod error;
pub use error::Error;
pub mod routes;
pub mod state;

pub fn trace() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
