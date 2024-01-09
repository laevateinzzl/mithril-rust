use std::{
    future::{Future, IntoFuture},
    process::Output,
    sync::{Arc, Mutex},
};

use axum::{
    extract,
    http::{request, StatusCode},
    middleware::FromExtractor,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::todo::service::TodoAppService,
    domain::entities::todo::{Priority, Status, Todo},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateTodoRequest {
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTodoRequest {
    id: i32,
}

// #[axum::debug_handler]
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

    (StatusCode::CREATED, Json(todo))
}
