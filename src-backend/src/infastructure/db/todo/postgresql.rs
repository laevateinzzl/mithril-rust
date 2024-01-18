use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{prelude::*, query};

use crate::domain::{entities::todo::Todo, repository::todo::TodoRepository};
pub struct PgSqlTodoRepository {
    pool: PgPool,
}

impl PgSqlTodoRepository {
    pub fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}

impl TodoRepository for PgSqlTodoRepository {
    fn get_all_by_user_id(
        &self,
        user_id: i32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<Todo>> + '_>> {
        let query = "SELECT * FROM todos WHERE user_id = ?";
        Box::pin(async move {
            if let Ok(todos) = sqlx::query_as::<_, Todo>(query)
                .bind(user_id)
                .fetch_all(&self.pool)
                .await
            {
                todos
            } else {
                Vec::new()
            }
        })
    }

    fn get_by_id(
        &self,
        id: i32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<Todo>> + '_>> {
        let query = "SELECT * FROM todos WHERE id = ?";
        Box::pin(async move {
            if let Ok(todo) = sqlx::query_as::<_, Todo>(query)
                .bind(id)
                .fetch_one(&self.pool)
                .await
            {
                Some(todo)
            } else {
                None
            }
        })
    }

    fn create(
        &self,
        todo: Todo,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Todo>>>> {
        let query = "INSERT INTO todos (user_id, title, description, status, priority, created_at, updated_at, deleted_at, deadline, done) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        Box::pin(async move {
            if let Ok(res) = sqlx::query(query)
                .bind(todo.user_id)
                .bind(todo.title.clone())
                .bind(todo.description.clone())
                .bind(todo.status)
                .bind(todo.priority)
                .bind(todo.created_at)
                .bind(todo.updated_at)
                .bind(todo.deleted_at)
                .bind(todo.deadline)
                .bind(todo.done)
                .fetch_one(&self.pool)
                .await
            {
                Ok(Todo {
                    id: res.try_get("id")?,
                    ..todo.clone()
                })
            } else {
                Err(anyhow::Error::msg("Error creating todo"))
            }
        })
    }

    fn save(&self, todo: Todo) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + '_>> {
        let query = "UPDATE todos SET user_id = ?, title = ?, description = ?, status = ?, priority = ?, created_at = ?, updated_at = ?, deleted_at = ?, deadline = ?, done = ? WHERE id = ?";
        Box::pin(async move {
            if let Ok(_) = sqlx::query(query)
                .bind(todo.user_id)
                .bind(todo.title)
                .bind(todo.description)
                .bind(todo.status)
                .bind(todo.priority)
                .bind(todo.created_at)
                .bind(todo.updated_at)
                .bind(todo.deleted_at)
                .bind(todo.deadline)
                .bind(todo.done)
                .execute(&self.pool)
                .await
            {
                true
            } else {
                false
            }
        })
    }

    fn delete(&self, id: i32) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + '_>> {
        let query = "DELETE FROM todos WHERE id = ?";
        Box::pin(async move {
            if let Ok(_) = sqlx::query(query).bind(id).execute(&self.pool).await {
                true
            } else {
                false
            }
        })
    }
}

// gen tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::todo::{Priority, Status};
    use chrono::Utc;
    use sqlx::postgres::PgConnectOptions;

    async fn setup() -> Result<PgSqlTodoRepository, sqlx::Error> {
        dotenv::dotenv().ok();
        let dns = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&dns)
            .await?;
        Ok(PgSqlTodoRepository::new(pool)?)
    }

    #[tokio::test]
    async fn test_get_all_by_user_id() {
        let repo = setup().await.expect("");
        let user_id = 1;
        let todos = repo.get_all_by_user_id(user_id).await;
        assert!(todos.is_empty()); // assuming no todos for user_id 1
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let repo = setup().await.expect("");
        let id = 1;
        let todo = repo.get_by_id(id).await;
        assert!(todo.is_none()); // assuming no todos for user_id 1
    }

    #[tokio::test]
    async fn test_create() {
        let repo = setup().await.expect("");
        let todo = Todo {
            id: 0,
            user_id: 1,
            title: "test".to_string(),
            description: "test".to_string(),
            status: Status::Open,
            priority: Priority::Low,
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: None,
            deadline: None,
            done: false,
        };
        let todo = repo.create(todo).await;
        assert!(todo.is_ok());
    }

    #[tokio::test]
    async fn test_save() {
        let repo = setup().await.expect("");
        let todo = Todo {
            id: 0,
            user_id: 1,
            title: "test".to_string(),
            description: "test".to_string(),
            status: Status::Open,
            priority: Priority::Low,
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: None,
            deadline: None,
            done: false,
        };
        let todo = repo.create(todo).await;
        assert!(todo.is_ok());
        let mut todo = todo.unwrap();
        todo.title = "test2".to_string();
        let result = repo.save(todo).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = setup().await.expect("");
        let todo = Todo {
            id: 0,
            user_id: 1,
            title: "test".to_string(),
            description: "test".to_string(),
            status: Status::Open,
            priority: Priority::Low,
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: None,
            deadline: None,
            done: false,
        };
        let todo = repo.create(todo).await;
        assert!(todo.is_ok());
        let todo = todo.unwrap();
        let result = repo.delete(todo.id).await;
        assert!(result);
    }
}
