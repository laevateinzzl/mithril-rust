use std::{
    future::{Future, IntoFuture},
    process::Output,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{self, path},
    http::{request, StatusCode},
    middleware::FromExtractor,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::request::Response,
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
    playload: Json<CreateTodoRequest>,
) -> impl IntoResponse {
    let todo = Todo {
        id: 0,
        user_id: 0,
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

    Response {
        code: 200,
        message: "success".to_string(),
        data: serde_json::to_value(todo).unwrap(),
    }
    // (StatusCode::CREATED, Json(todo))
}

pub async fn get_todo(
    todo_service: extract::Extension<Arc<dyn TodoAppService>>,
    path::Path(id): path::Path<i32>,
) -> impl IntoResponse {
    let todo = todo_service.get_by_id(id).await;

    // (StatusCode::OK, Json(todo))
    Response {
        code: 200,
        message: "success".to_string(),
        data: serde_json::to_value(todo).unwrap(),
    }
}
