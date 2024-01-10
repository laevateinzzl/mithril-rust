use chrono::{DateTime, Local};
use sqlx::{Decode, Encode, FromRow, Row, Type};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Type)]
#[repr(i16)]
pub enum Status {
    Open = 1,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Type)]
#[repr(i16)]
pub enum Priority {
    Low = 1,
    Medium,
    High,
}

#[derive(Debug, Clone, Encode, serde::Serialize, serde::Deserialize)]
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

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Todo {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let status: i32 = row.try_get("status")?;
        let status_enum = match status {
            1 => Status::Open,
            2 => Status::InProgress,
            3 => Status::Done,
            _ => Status::Open,
        };
        let priority: i32 = row.try_get("priority")?;
        let priority_enum = match priority {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::Low,
        };

        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            status: status_enum,
            priority: priority_enum,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            deadline: row.try_get("deadline")?,
            done: row.try_get("done")?,
        })
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> for Todo {
    fn from_row(row: &'r sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        let status: i32 = row.try_get("status")?;
        let status_enum = match status {
            1 => Status::Open,
            2 => Status::InProgress,
            3 => Status::Done,
            _ => Status::Open,
        };
        let priority: i32 = row.try_get("priority")?;
        let priority_enum = match priority {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::Low,
        };

        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            status: status_enum,
            priority: priority_enum,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
            deadline: row.try_get("deadline")?,
            done: row.try_get("done")?,
        })
    }
}
