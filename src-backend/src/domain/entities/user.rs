use chrono::{DateTime, Local};
use sqlx::{Encode, FromRow};

#[derive(Debug, Clone, Encode, serde::Serialize, serde::Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        email: String,
        salt: String,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
        deleted_at: Option<DateTime<Local>>,
    ) -> Self {
        Self {
            id: 0,
            username,
            password,
            email,
            salt,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
