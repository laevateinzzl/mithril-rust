use crate::domain::entities::user::User;
use anyhow::Result;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: i32) -> Option<User>;
    async fn get_by_email(&self, email: String) -> Option<User>;
    async fn create(&self, user: &User) -> Result<User>;
    async fn save(&self, user: User) -> bool;
    async fn delete(&self, id: i32) -> bool;
}
