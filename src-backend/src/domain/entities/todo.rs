use chrono::{DateTime, Local};
use sqlx::{Decode, Encode, FromRow, Type};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Type)]
#[repr(i32)]
pub enum Status {
    Open = 1,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Type)]
#[repr(i32)]
pub enum Priority {
    Low = 1,
    Medium,
    High,
}

#[derive(Debug, Clone, Decode, Encode, FromRow, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
    pub deadline: Option<DateTime<Local>>,
    pub done: bool,
}

impl Todo {
    pub fn new(
        user_id: i32,
        title: String,
        description: String,
        status: Status,
        priority: Priority,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
        deleted_at: Option<DateTime<Local>>,
        deadline: Option<DateTime<Local>>,
        done: bool,
    ) -> Self {
        Self {
            id: 0,
            user_id,
            title,
            description,
            status,
            priority,
            created_at,
            updated_at,
            deleted_at,
            deadline,
            done,
        }
    }
}
