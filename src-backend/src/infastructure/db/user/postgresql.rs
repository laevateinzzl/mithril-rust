use anyhow::Result;
use sqlx::{PgPool, Row};

use crate::domain::{entities::user::User, repository::user::UserRepository};

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn get_by_id(&self, id: i32) -> Option<User> {
        let query = "SELECT * FROM user WHERE id = $1";
        if let Ok(user) = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Some(user)
        } else {
            None
        }
    }

    async fn get_by_email(&self, email: String) -> Option<User> {
        let query = "SELECT * FROM user WHERE email = $1";
        if let Ok(user) = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_one(&self.pool)
            .await
        {
            Some(user)
        } else {
            None
        }
    }

    async fn create(&self, user: &User) -> Result<User> {
        let query = "INSERT INTO user (email, password, salt, created_at, updated_at, deleted_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
        if let Ok(res) = sqlx::query(query)
            .bind(user.email.clone())
            .bind(user.password.clone())
            .bind(user.salt.clone())
            .bind(user.created_at)
            .bind(user.updated_at)
            .bind(user.deleted_at)
            .fetch_one(&self.pool)
            .await
        {
            Ok(User {
                id: res.try_get("id")?,
                ..user.clone()
            })
        } else {
            Err(anyhow::anyhow!("Error creating user"))
        }
    }

    async fn save(&self, user: User) -> bool {
        let query = "UPDATE user SET email = $1, password = $2, salt = $3, updated_at = $4, deleted_at = $5 WHERE id = $6";
        if let Ok(_) = sqlx::query(query)
            .bind(user.email)
            .bind(user.password)
            .bind(user.salt.clone())
            .bind(user.updated_at)
            .bind(user.deleted_at)
            .bind(user.id)
            .execute(&self.pool)
            .await
        {
            true
        } else {
            false
        }
    }

    async fn delete(&self, id: i32) -> bool {
        let query = "DELETE FROM user WHERE id = $1";
        if let Ok(_) = sqlx::query(query).bind(id).execute(&self.pool).await {
            true
        } else {
            false
        }
    }
}
