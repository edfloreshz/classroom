use crate::{
    database::entities::user::{Role, User},
    models::user::{GetUserParams, PostUserParams, PutUserParams},
    prelude::*,
};
use sqlx::{query_as, Pool, Sqlite};

pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<User>, Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get(pool: &Pool<Sqlite>, params: GetUserParams) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(params.id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_email(pool: &Pool<Sqlite>, email: String) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn post(pool: &Pool<Sqlite>, user: PostUserParams) -> Result<(), Error> {
    query!(
        "INSERT INTO users
        (id, username, email, password, first_name, last_name, role)
        VALUES
        (?, ?, ?, ?, ?, ?, ?)",
        user.id,
        user.username,
        user.email,
        user.password,
        user.first_name,
        user.last_name,
        Role::Student,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn put(pool: &Pool<Sqlite>, params: PutUserParams) -> Result<(), Error> {
    let role: Role = params.role.into();
    query!(
        "UPDATE users
        SET username = ?, email = ?, first_name = ?, last_name = ?, role = ?, active = ?
        WHERE id = ?",
        params.username,
        params.email,
        params.first_name,
        params.last_name,
        role,
        params.active,
        params.id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete(pool: &Pool<Sqlite>, params: GetUserParams) -> Result<(), Error> {
    query!("DELETE FROM users WHERE id = ?", params.id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
pub struct Exists {
    pub row_exists: bool,
}

pub async fn user_exists(pool: &Pool<Sqlite>, email: &impl ToString) -> Result<bool, Error> {
    let result =
        query_as::<_, Exists>("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) AS row_exists")
            .bind(email.to_string())
            .fetch_one(pool)
            .await?;
    Ok(result.row_exists)
}
