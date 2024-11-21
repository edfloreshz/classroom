use garde::Validate;
use sqlx::{Pool, Sqlite};

use crate::prelude::*;

#[derive(sqlx::FromRow, Default, Validate, Clone, Deserialize, Serialize)]
pub struct User {
    #[garde(skip)]
    pub id: String,
    #[garde(skip)]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub password: String,
    #[garde(skip)]
    pub first_name: String,
    #[garde(skip)]
    pub last_name: Option<String>,
    #[garde(skip)]
    pub created_at: DateTime<Utc>,
    #[garde(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UserParams {
    id: String,
}

pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get(pool: &Pool<Sqlite>, params: UserParams) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(params.id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(pool: &Pool<Sqlite>, email: String) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn post(pool: &Pool<Sqlite>, user: User) -> Result<(), Error> {
    query!(
        "INSERT INTO users
        (id, username, email, password, first_name, last_name)
        VALUES
        (?, ?, ?, ?, ?, ?)",
        user.id,
        user.username,
        user.email,
        user.password,
        user.first_name,
        user.last_name,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn put(pool: &Pool<Sqlite>, user: User) -> Result<(), Error> {
    query!(
        "UPDATE users
        SET username = ?, email = ?, password = ?, first_name = ?, last_name = ?
        WHERE id = ?",
        user.username,
        user.email,
        user.password,
        user.first_name,
        user.last_name,
        user.id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &Pool<Sqlite>, params: UserParams) -> Result<(), Error> {
    query("DELETE FROM users WHERE id = ?")
        .bind(params.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn user_exists(pool: &Pool<Sqlite>, email: &impl ToString) -> Result<bool, Error> {
    let result = query("SELECT * FROM users WHERE email = ?")
        .bind(email.to_string())
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}