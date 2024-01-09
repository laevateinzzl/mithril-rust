use std::sync::{Arc, Mutex};

use axum::{routing::post, Extension, Router};

use crate::{
    application::todo::service::TodoAppServiceImpl, infastructure::db::mysql::MySqlTodoRepository,
};

use super::todo::api::create_todo;

const MYSQL_DSN: &str = "mysql://root:password@localhost:3306/todo";

// #[axum::debug_handler]
pub async fn create_router() -> Router {
    let todo_repo = MySqlTodoRepository::new(MYSQL_DSN.to_string()).await;

    let todo_repo = match todo_repo {
        Ok(repo) => repo,
        Err(error) => {
            panic!("Failed to create TodoRepository: {}", error)
        }
    };

    let todo_service = Arc::new(Mutex::new(TodoAppServiceImpl::new(todo_repo)));

    Router::new()
        .route("/api/todo", post(create_todo))
        .layer(Extension(Arc::new(todo_service)))
}
