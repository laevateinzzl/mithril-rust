use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::{
    application::todo::service::{TodoAppService, TodoAppServiceImpl},
    infastructure::db::mysql::MySqlTodoRepository,
};

use super::todo::api::create_todo;

pub async fn create_router() -> Router {
    // get database url from .env file
    dotenv::dotenv().ok();

    let database_url = match std::env::var("MYSQL_DSN") {
        Ok(url) => url,
        Err(error) => {
            panic!("Failed to get DATABASE_URL from .env file: {}", error)
        }
    };

    let todo_repo = MySqlTodoRepository::new(database_url).await;

    let todo_repo = match todo_repo {
        Ok(repo) => repo,
        Err(error) => {
            panic!("Failed to create TodoRepository: {}", error)
        }
    };

    // let todo_service = Arc::new(TodoAppServiceImpl::new(todo_repo));
    let todo_service: Arc<dyn TodoAppService> = Arc::new(TodoAppServiceImpl::new(todo_repo));

    Router::new()
        .route("/api/todo", post(create_todo))
        .layer(Extension(todo_service))
}
