use sqlx::MySqlPool;

use crate::domain::{entities::todo::Todo, repository::todo::TodoRepository};

pub struct MySqlTodoRepository {
    pool: MySqlPool,
}

impl MySqlTodoRepository {
    pub async fn new(dsn: String) -> Result<Self, sqlx::Error> {
        let pool = MySqlPool::connect(dsn.as_str()).await?;
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
            return Vec::new();
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
    async fn create(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        let query = "INSERT INTO todos (user_id, title, description, status, priority, created_at, updated_at, deleted_at, deadline, done) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        if let Ok(todo) = sqlx::query_as::<_, Todo>(query)
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
            .fetch_one(&self.pool)
            .await
        {
            Ok(todo)
        } else {
            Err(sqlx::Error::RowNotFound)
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
