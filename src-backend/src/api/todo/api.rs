use std::sync::Arc;

use axum::{
    extract::{self, path},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::request::{error_response, success_response},
    application::todo::service::TodoAppService,
    domain::entities::todo::{Priority, Status, Todo},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateTodoRequest {
    title: String,
    description: String,
}

#[axum::debug_handler]
pub async fn create_todo(
    todo_service: extract::Extension<Arc<dyn TodoAppService>>,
    Extension(user_id): Extension<i32>,
    playload: Json<CreateTodoRequest>,
) -> impl IntoResponse {
    let todo = Todo {
        id: 0,
        user_id,
        title: playload.title.clone(),
        description: playload.description.clone(),
        status: Status::Open,
        priority: Priority::Low,
        created_at: chrono::Local::now(),
        updated_at: chrono::Local::now(),
        deleted_at: None,
        deadline: None,
        done: false,
    };

    let todo = todo_service.create(todo).await;
    if let Ok(todo) = todo {
        success_response(serde_json::to_value(todo).unwrap())
    } else {
        error_response(500, "Failed to create todo".to_string())
    }
}

pub async fn get_todo(
    todo_service: extract::Extension<Arc<dyn TodoAppService>>,
    path::Path(id): path::Path<i32>,
) -> impl IntoResponse {
    let todo = todo_service.get_by_id(id).await;
    print!("{id}");
    if let Some(todo) = todo {
        success_response(serde_json::to_value(todo).unwrap())
    } else {
        error_response(500, "Failed to get todo".to_string())
    }
}

pub async fn get_todo_list(
    todo_service: extract::Extension<Arc<dyn TodoAppService>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    let todo_list = todo_service.get_all_by_user_id(user_id).await;
    success_response(serde_json::to_value(todo_list).unwrap())
}
