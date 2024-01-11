use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};

use crate::application::todo::service::{TodoAppService, TodoAppServiceImpl};

use super::todo::api::{create_todo, get_todo};

pub async fn create_router() -> Router {
    // get database url from .env file
    dotenv::dotenv().ok();

    // 读取变量DB_TYPE的值,初始化对应pool
    let db_type = std::env::var("DB_TYPE").expect("DB_TYPE must be set");

    // let todo_service = Arc::new(TodoAppServiceImpl::new(todo_repo));
    let todo_service: Arc<dyn TodoAppService> = Arc::new(TodoAppServiceImpl::new(todo_repo));

    Router::new()
        .route("/api/todo", post(create_todo))
        .route("/api/todo/:id", get(get_todo))
        .layer(Extension(todo_service))
}
