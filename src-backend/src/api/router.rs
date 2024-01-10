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

    // 这里看配置文件里面是mysql的链接还是pgsql的链接，然后选择对应的repo
    let mysql_dsn = std::env::var("MYSQL_DSN");
    let pgsql_dsn = std::env::var("PGSQL_DSN");

    let todo_repo = if mysql_dsn.is_ok() {
        let dsn = mysql_dsn.unwrap();
        MySqlTodoRepository::new(dsn)
            .await
            .expect("Failed to create repository")
    } else if pgsql_dsn.is_ok() {
        let dsn = pgsql_dsn.unwrap();
        MySqlTodoRepository::new(dsn)
            .await
            .expect("Failed to create repository")
    } else {
        panic!("No database url found in .env file")
    };

    // let todo_service = Arc::new(TodoAppServiceImpl::new(todo_repo));
    let todo_service: Arc<dyn TodoAppService> = Arc::new(TodoAppServiceImpl::new(todo_repo));

    Router::new()
        .route("/api/todo", post(create_todo))
        .layer(Extension(todo_service))
}
