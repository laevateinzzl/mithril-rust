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

#[async_trait::async_trait]
impl TodoRepository for PgSqlTodoRepository {
    async fn get_all_by_user_id(&self, user_id: i32) -> Vec<Todo> {
        let query = "SELECT * FROM todos WHERE user_id = $1 RETURNING *";
        if let Ok(todos) = sqlx::query_as::<_, Todo>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
        {
            todos
        } else {
            Vec::new()
        }
    }
    async fn get_by_id(&self, id: i32) -> Option<Todo> {
        let query = "SELECT * FROM todos WHERE id = $1 RETURNING *";
        if let Ok(todo) = sqlx::query_as::<_, Todo>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Some(todo)
        } else {
            None
        }
    }

    async fn create(&self, todo: &Todo) -> Result<Todo> {
        let query = "INSERT INTO todos (user_id, title, description, status, priority, created_at, updated_at, deleted_at, deadline, done) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *";

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
                user_id: res.try_get("user_id")?,
                title: res.try_get("title")?,
                description: res.try_get("description")?,
                status: res.try_get("status")?,
                priority: res.try_get("priority")?,
                created_at: res.try_get("created_at")?,
                updated_at: res.try_get("updated_at")?,
                deleted_at: res.try_get("deleted_at")?,
                deadline: res.try_get("deadline")?,
                done: res.try_get("done")?,
            })
        } else {
            Err(anyhow::anyhow!("Failed to create todo"))
        }
    }
    async fn save(&self, todo: Todo) -> Result<bool, sqlx::Error> {
        let query = "UPDATE todos SET user_id = $1, title = $2, description = $3, status = $4, priority = $5, created_at = $6, updated_at = $7, deleted_at = $8, deadline = $9, done = $10 WHERE id = $11";
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
            .bind(todo.id)
            .execute(&self.pool)
            .await
        {
            Ok(true)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
    async fn delete(&self, id: i32) -> bool {
        let query = "DELETE FROM todos WHERE id = $1";
        if let Ok(_) = sqlx::query(query).bind(id).execute(&self.pool).await {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::domain::entities::todo::{Priority, Status};

    use super::*;
    use chrono::Local;

    async fn setup() -> PgSqlTodoRepository {
        dotenv::dotenv().ok();
        let dsn = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&dsn)
            .await
            .unwrap();
        PgSqlTodoRepository::new(pool).unwrap()
    }

    #[tokio::test]
    async fn test_get_all_by_user_id() {
        let repo = setup().await;
        let todos = repo.get_all_by_user_id(1).await;
        assert_eq!(todos.len(), 1);
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let repo = setup().await;
        let todo = repo.get_by_id(11).await;
        assert!(todo.is_some());
    }

    // #[tokio::test]
    #[sqlx::test]
    async fn test_create() {
        let repo = setup().await;
        let todo = Todo::new(
            1,
            "Test".to_string(),
            "Test".to_string(),
            Status::Open,
            Priority::Low,
            Local::now(),
            Local::now(),
            None,
            None,
            false,
        );
        let todo = repo.create(&todo).await.unwrap();
        assert!(todo.id > 0);
    }
}
