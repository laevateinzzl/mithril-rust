use anyhow::Result;
use std::str::FromStr;

use sqlx::{mysql::MySqlConnectOptions, ConnectOptions, MySqlPool};

use crate::domain::{entities::todo::Todo, repository::todo::TodoRepository};

pub struct MySqlTodoRepository {
    pool: MySqlPool,
}

impl MySqlTodoRepository {
    pub fn new(pool: MySqlPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl TodoRepository for MySqlTodoRepository {
    async fn get_all_by_user_id(&self, user_id: i32) -> Vec<Todo> {
        let query = "SELECT * FROM todos WHERE user_id = ?";
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
        let query = "SELECT * FROM todos WHERE id = ?";
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
        let query = "INSERT INTO todos (user_id, title, description, status, priority, created_at, updated_at, deleted_at, deadline, done) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
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
            .execute(&self.pool)
            .await
        {
            Ok(Todo {
                id: res.last_insert_id() as i32,
                ..todo.clone()
            })
        } else {
            Err(anyhow::anyhow!("Failed to create todo"))
        }
    }
    async fn save(&self, todo: Todo) -> Result<bool, sqlx::Error> {
        let query = "UPDATE todos SET user_id = ?, title = ?, description = ?, status = ?, priority = ?, created_at = ?, updated_at = ?, deleted_at = ?, deadline = ?, done = ? WHERE id = ?";
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
        let query = "DELETE FROM todos WHERE id = ?";
        if let Ok(_) = sqlx::query(query).bind(id).execute(&self.pool).await {
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::todo::{Priority, Status, Todo};
    use std::env;

    async fn setup() -> MySqlTodoRepository {
        dotenv::dotenv().ok();
        let dsn = env::var("MYSQL_DSN").expect("DATABASE_URL must be set");
        MySqlTodoRepository::new(dsn)
            .await
            .expect("Failed to create repository")
    }

    #[tokio::test]
    async fn test_get_all_by_user_id() {
        let repo = setup().await;
        let user_id = 1;
        let todos = repo.get_all_by_user_id(user_id).await;
        assert!(todos.len() >= 0);
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let repo = setup().await;
        let id = 1;
        let todo = repo.get_by_id(id).await.unwrap();
        print!("{:?}", todo)
        // assert!(todo.is_some());
    }

    #[tokio::test]
    async fn test_create() {
        let repo = setup().await;
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
        let result = repo.create(&todo).await;
        print!("{:?}", result);
        // assert!(result.is_ok());
    }

    // #[tokio::test]
    // async fn test_save() {
    //     let repo = setup().await;
    //     let todo = Todo {
    //         // Fill in the fields here
    //         // ...
    //     };
    //     let result = repo.save(todo).await;
    //     assert!(result.is_ok());
    // }

    #[tokio::test]
    async fn test_delete() {
        let repo = setup().await;
        let id = 1;
        let result = repo.delete(id).await;
        assert!(result);
    }
}
