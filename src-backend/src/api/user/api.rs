use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}
