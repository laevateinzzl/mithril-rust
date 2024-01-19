use std::sync::Arc;

use axum::response::IntoResponse;
use axum::{extract, Json};
use serde::{Deserialize, Serialize};

use crate::api::request::{error_response, success_response};
use crate::application::user::service::UserService;

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

pub async fn register(
    user_service: extract::Extension<Arc<dyn UserService>>,
    playload: Json<CreateUserRequest>,
) -> impl IntoResponse {
    // get request body from playload
    let req = playload.0.clone();
    let res = user_service.register(req).await;
    if let Ok(user) = res {
        success_response(serde_json::to_value(user).unwrap())
    } else {
        error_response(500, "Failed to register user".to_string())
    }
}

pub async fn login(
    user_service: extract::Extension<Arc<dyn UserService>>,
    playload: Json<UserLoginRequest>,
) -> impl IntoResponse {
    // get request body from playload
    let req = playload.0.clone();
    let res = user_service.login(req).await;
    if let Ok(user) = res {
        success_response(serde_json::to_value(user).unwrap())
    } else {
        error_response(500, "Failed to login user".to_string())
    }
}
