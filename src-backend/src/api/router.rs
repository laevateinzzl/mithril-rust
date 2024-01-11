use std::sync::Arc;

use axum::{
    routing::{get, post},
    Extension, Router,
};

use crate::{
    application::todo::service::{TodoAppService, TodoAppServiceImpl},
    domain::repository::todo::TodoRepository,
    infastructure::db::{
        init_db,
        todo::{mysql::MySqlTodoRepository, postgresql::PgSqlTodoRepository},
        Database, DB,
    },
};

use super::todo::api::{create_todo, get_todo};

fn create_todo_service<T>(todo_repository: T) -> Arc<dyn TodoAppService>
where
    T: TodoRepository + 'static,
{
    Arc::new(TodoAppServiceImpl::new(todo_repository))
}

pub async fn create_router() -> Router {
    init_db().await;
    let db = DB.lock().unwrap();
    if let Some(database) = &*db {
        let todo_service: Arc<dyn TodoAppService> = match database {
            Database::MySQL(pool) => {
                let todo_repository = MySqlTodoRepository::new(pool.clone()).unwrap();
                create_todo_service(todo_repository)
            }
            Database::PgSQL(pool) => {
                let todo_repository = PgSqlTodoRepository::new(pool.clone()).unwrap();
                create_todo_service(todo_repository)
            }
        };

        return Router::new()
            .route("/api/todo", post(create_todo))
            .route("/api/todo/:id", get(get_todo))
            .layer(Extension(todo_service));
    } else {
        panic!("Database not initialized");
    }
}
