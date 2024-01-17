use std::{future::Future, pin::Pin};

use crate::domain::entities::user::User;
use anyhow::Result;

// #[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    fn get_by_id(&self, id: i32) -> Pin<Box<dyn Future<Output = Option<User>> + '_>>;
    fn get_by_email(&self, email: String) -> Pin<Box<dyn Future<Output = Option<User>> + '_>>;
    fn create(&self, user: User) -> Pin<Box<dyn Future<Output = Result<User>> + '_>>;
    fn save(&self, user: User) -> Pin<Box<dyn Future<Output = bool> + '_>>;
    fn delete(&self, id: i32) -> Pin<Box<dyn Future<Output = bool> + '_>>;
}
