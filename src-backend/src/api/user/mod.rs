use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}
