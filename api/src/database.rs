use std::env;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::prelude::*;

pub async fn connection_pool() -> Result<Pool<Sqlite>, Error> {
    let database_url = env::var("DATABASE_URL")?;

    std::fs::create_dir_all("database")?;
    std::fs::File::create_new("database/database.db")?;

    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
