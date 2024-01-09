use sqlx::Error;

use crate::domain::entities::todo::Todo;

#[async_trait::async_trait]
pub trait TodoRepository: Sync {
    async fn get_all_by_user_id(&self, user_id: i32) -> Vec<Todo>;
    async fn get_by_id(&self, id: i32) -> Option<Todo>;
    async fn create(&self, todo: Todo) -> Result<Todo, Error>;
    async fn save(&self, todo: Todo) -> Result<bool, Error>;
    async fn delete(&self, id: i32) -> bool;
}
