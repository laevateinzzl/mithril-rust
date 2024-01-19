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
    async fn update_user(&self, user: User) -> bool;
}

pub struct UserServiceImpl<T> {
    user_repository: T,
}

#[async_trait::async_trait]
impl<T: UserRepository> UserService for UserServiceImpl<T> {
    async fn register(&self, req: CreateUserRequest) -> Result<User> {
        if req.password != req.password_confirmation {
            return Err(anyhow::anyhow!("password not match"));
        }

        let hasher = crate::utils::encryption::password::PasswordHasher::new();
        let (password_hash, salt) = hasher.hash_password(req.password.as_str())?;

        self.user_repository
            .create(&User::new(
                req.username,
                password_hash,
                req.email,
                salt,
                chrono::Local::now(),
                chrono::Local::now(),
                None,
            ))
            .await
    }
    async fn login(&self, req: UserLoginRequest) -> Result<User> {
        let user = self
            .user_repository
            .get_by_email(req.email.clone())
            .await
            .ok_or(anyhow::anyhow!("user not found"))?;
        let hasher = crate::utils::encryption::password::PasswordHasher::new();
        let is_valid =
            hasher.verify_password(req.password.as_str(), &user.salt, user.password.as_str());
        if !is_valid {
            return Err(anyhow::anyhow!("password not match"));
        }
        Ok(user)
    }
    async fn get_user_by_id(&self, id: i32) -> Option<User> {
        self.user_repository.get_by_id(id).await
    }
    async fn get_user_by_email(&self, email: String) -> Option<User> {
        self.user_repository.get_by_email(email).await
    }
    async fn get_user_by_token(&self, token: String) -> Option<User> {
        // self.user_repository.get_by_token(token).await
        todo!()
    }
    async fn update_user(&self, user: User) -> bool {
        self.user_repository.save(user).await
    }
}
