use anyhow::Result;
use sqlx::MySqlPool;

use crate::domain::{entities::user::User, repository::user::UserRepository};

pub struct MySqlUserRepository {
    pool: MySqlPool,
}

impl MySqlUserRepository {
    pub fn new(pool: MySqlPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }
}

impl UserRepository for MySqlUserRepository {
    fn get_by_id(
        &self,
        id: i32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<User>> + '_>> {
        let query = "SELECT * FROM users WHERE id = ?";
        Box::pin(async move {
            if let Ok(user) = sqlx::query_as::<_, User>(query)
                .bind(id)
                .fetch_one(&self.pool)
                .await
            {
                Some(user)
            } else {
                None
            }
        })
    }

    fn get_by_email(
        &self,
        email: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<User>> + '_>> {
        let query = "SELECT * FROM users WHERE email = ?";
        Box::pin(async move {
            if let Ok(user) = sqlx::query_as::<_, User>(query)
                .bind(email)
                .fetch_one(&self.pool)
                .await
            {
                Some(user)
            } else {
                None
            }
        })
    }

    fn create(
        &self,
        user: User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<User>> + '_>> {
        let query = "INSERT INTO users (email, password, created_at, updated_at, deleted_at) VALUES (?, ?, ?, ?, ?)";
        Box::pin(async move {
            if let Ok(res) = sqlx::query(query)
                .bind(user.email.clone())
                .bind(user.password.clone())
                .bind(user.created_at)
                .bind(user.updated_at)
                .bind(user.deleted_at)
                .execute(&self.pool)
                .await
            {
                Ok(User {
                    id: res.last_insert_id() as i32,
                    ..user.clone()
                })
            } else {
                Err(anyhow::anyhow!("Error creating user"))
            }
        })
    }

    fn save(&self, user: User) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + '_>> {
        let query =
            "UPDATE users SET email = ?, password = ?, updated_at = ?, deleted_at = ? WHERE id = ?";
        Box::pin(async move {
            if let Ok(_) = sqlx::query(query)
                .bind(user.email.clone())
                .bind(user.password.clone())
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
        })
    }

    fn delete(&self, id: i32) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + '_>> {
        let query = "DELETE FROM users WHERE id = ?";
        Box::pin(async move {
            if let Ok(_) = sqlx::query(query).bind(id).execute(&self.pool).await {
                true
            } else {
                false
            }
        })
    }
}

// #[async_trait::async_trait]
// impl UserRepository for MySqlUserRepository {
//     async fn get_by_id(&self, id: i32) -> Option<User> {
//         let query = "SELECT * FROM users WHERE id = ?";
//         if let Ok(user) = sqlx::query_as::<_, User>(query)
//             .bind(id)
//             .fetch_one(&self.pool)
//             .await
//         {
//             Some(user)
//         } else {
//             None
//         }
//     }

//     async fn get_by_email(&self, email: String) -> Option<User> {
//         let query = "SELECT * FROM users WHERE email = ?";
//         if let Ok(user) = sqlx::query_as::<_, User>(query)
//             .bind(email)
//             .fetch_one(&self.pool)
//             .await
//         {
//             Some(user)
//         } else {
//             None
//         }
//     }

//     async fn create(&self, user: &User) -> Result<User> {
//         let query = "INSERT INTO users (email, password, created_at, updated_at, deleted_at) VALUES (?, ?, ?, ?, ?)";
//         if let Ok(res) = sqlx::query(query)
//             .bind(user.email.clone())
//             .bind(user.password.clone())
//             .bind(user.created_at)
//             .bind(user.updated_at)
//             .bind(user.deleted_at)
//             .execute(&self.pool)
//             .await
//         {
//             Ok(User {
//                 id: res.last_insert_id() as i32,
//                 ..user.clone()
//             })
//         } else {
//             Err(anyhow::anyhow!("Error creating user"))
//         }
//     }

//     async fn save(&self, user: User) -> bool {
//         let query =
//             "UPDATE users SET email = ?, password = ?, updated_at = ?, deleted_at = ? WHERE id = ?";
//         if let Ok(_) = sqlx::query(query)
//             .bind(user.email.clone())
//             .bind(user.password.clone())
//             .bind(user.updated_at)
//             .bind(user.deleted_at)
//             .bind(user.id)
//             .execute(&self.pool)
//             .await
//         {
//             true
//         } else {
//             false
//         }
//     }

//     async fn delete(&self, id: i32) -> bool {
//         let query = "DELETE FROM users WHERE id = ?";
//         if let Ok(_) = sqlx::query(query).bind(id).execute(&self.pool).await {
//             true
//         } else {
//             false
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::User;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;

    async fn setup() -> Result<MySqlUserRepository, sqlx::Error> {
        let dsn = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&dsn)
            .await
            .expect("Failed to connect to MySQL");
        Ok(MySqlUserRepository::new(pool)?)
    }

    #[tokio::test]
    async fn test_save() {
        let repo = setup().await.expect("Failed to setup repository");
        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: None,
        };
        let result = repo.save(user).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = setup().await.expect("Failed to setup repository");
        let id = 1;
        let result = repo.delete(id).await;
        assert!(result);
    }
}
