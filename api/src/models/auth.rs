use crate::{database::entities::user::Role, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub role: Role,
}

#[derive(Deserialize, Validate)]
pub struct ActivationParams {
    #[garde(email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct SignInParams {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
#[garde(allow_unvalidated)]
pub struct RegisterParams {
    #[garde(length(min = 5))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 15))]
    pub password: String,
    pub first_name: String,
    pub last_name: Option<String>,
}
