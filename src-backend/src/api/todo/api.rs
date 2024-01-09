use std::sync::{Arc, Mutex};

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    application::todo::service::TodoAppService,
    domain::entities::todo::{Priority, Status, Todo},
};

#[derive(Deserialize, Serialize)]
struct CreateTodoRequest {
    title: String,
    description: String,
}

#[derive(Deserialize, Serialize)]
struct GetTodoRequest {
    id: i32,
}

pub struct TodoController {
    todo_app_service: Arc<Mutex<dyn TodoAppService>>,
}

impl TodoController {
    pub fn new(todo_app_service: Arc<Mutex<dyn TodoAppService>>) -> Self {
        Self { todo_app_service }
    }

    async fn create(&self, request: Json<CreateTodoRequest>) -> impl IntoResponse {
        let todo = self
            .todo_app_service
            .lock()
            .unwrap()
            .create(Todo {
                id: 0,
                user_id: 0,
                title: request.title.clone(),
                description: request.description.clone(),
                status: Status::Open,
                priority: Priority::Medium,
                created_at: chrono::Local::now(),
                updated_at: chrono::Local::now(),
                deleted_at: None,
                deadline: None,
                done: false,
            })
            .await;
        Json(todo)
    }
}
