use std::env;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::Error;

pub async fn pool() -> Result<Pool<Sqlite>, Error> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
