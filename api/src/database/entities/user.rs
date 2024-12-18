use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::user;

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

#[derive(
    sqlx::Type, Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd,
)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin = 0,
    Teacher = 1,
    Student = 2,
    #[default]
    Generic = 3,
}

impl Role {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl From<user::Role> for Role {
    fn from(role: user::Role) -> Self {
        match role {
            user::Role::Admin => Role::Admin,
            user::Role::Teacher => Role::Teacher,
            user::Role::Student => Role::Student,
            user::Role::Generic => Role::Generic,
        }
    }
}
