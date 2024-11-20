use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

impl AppState {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}
