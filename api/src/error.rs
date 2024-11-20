use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("SQLx: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("EnvVar: {0}")]
    Var(#[from] std::env::VarError),
    #[error("Migration: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}
