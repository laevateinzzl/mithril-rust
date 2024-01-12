use ::anyhow::Result;

use crate::{
    api::user::api::{CreateUserRequest, UserLoginRequest},
    domain::{entities::user::User, repository::user::UserRepository},
};

#[async_trait::async_trait()]
pub trait UserService: Send + Sync {
    async fn register(&self, req: CreateUserRequest) -> Result<User>;
    async fn login(&self, req: UserLoginRequest) -> Result<User>;
    async fn get_user_by_id(&self, id: i32) -> Option<User>;
    async fn get_user_by_email(&self, email: String) -> Option<User>;
    async fn get_user_by_token(&self, token: String) -> Option<User>;
    async fn update_user(&self, user: User) -> Result<User>;
}

pub struct UserServiceImpl<T> {
    user_repository: T,
}

#[async_trait::async_trait]
impl<T: UserRepository> UserService for UserServiceImpl<T> {
    async fn register(&self, req: CreateUserRequest) -> Result<User> {
        todo!()
    }
    async fn login(&self, req: UserLoginRequest) -> Result<User> {
        todo!()
    }
    async fn get_user_by_id(&self, id: i32) -> Option<User> {
        todo!()
    }
    async fn get_user_by_email(&self, email: String) -> Option<User> {
        todo!()
    }
    async fn get_user_by_token(&self, token: String) -> Option<User> {
        todo!()
    }
    async fn update_user(&self, user: User) -> Result<User> {
        todo!()
    }
}
