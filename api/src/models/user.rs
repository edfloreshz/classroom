use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::services::entities::user::Role;

#[derive(Debug, Deserialize)]
pub struct GetUserParams {
    pub id: String,
}

#[derive(Default, Validate, Clone, Deserialize, Serialize)]
#[garde(allow_unvalidated)]
pub struct UserParams {
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
}
