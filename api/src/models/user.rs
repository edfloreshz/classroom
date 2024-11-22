use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::database::entities::user;

#[derive(sqlx::FromRow, Default, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: Role,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role.into(),
            active: user.active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl User {
    pub fn from_vec(users: Vec<user::User>) -> Vec<User> {
        users.into_iter().map(User::from).collect()
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum Role {
    Admin = 0,
    Teacher = 1,
    #[default]
    Student = 2,
}

impl From<user::Role> for Role {
    fn from(role: user::Role) -> Self {
        match role {
            user::Role::Admin => Role::Admin,
            user::Role::Teacher => Role::Teacher,
            user::Role::Student => Role::Student,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetUserParams {
    pub id: String,
}

#[derive(Default, Validate, Clone, Deserialize, Serialize)]
#[garde(allow_unvalidated)]
pub struct PostUserParams {
    pub id: String,
    #[garde(length(min = 5))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
#[garde(allow_unvalidated)]
pub struct PutUserParams {
    pub id: String,
    #[garde(length(min = 5))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: Role,
    pub active: bool,
}

impl From<user::User> for PutUserParams {
    fn from(user: user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role.into(),
            active: user.active,
        }
    }
}
