use std::{future::Future, pin::Pin};

use anyhow::Result;
use sqlx::Error;

use crate::domain::entities::todo::Todo;

// #[async_trait::async_trait]
pub trait TodoRepository: Send + Sync {
    fn get_all_by_user_id(&self, user_id: i32) -> Pin<Box<dyn Future<Output = Vec<Todo>> + '_>>;
    fn get_by_id(&self, id: i32) -> Pin<Box<dyn Future<Output = Option<Todo>> + '_>>;
    fn create(&self, todo: Todo) -> Pin<Box<dyn Future<Output = Result<Todo>>>>;
    fn save(&self, todo: Todo) -> Pin<Box<dyn Future<Output = bool> + '_>>;
    fn delete(&self, id: i32) -> Pin<Box<dyn Future<Output = bool> + '_>>;
}
