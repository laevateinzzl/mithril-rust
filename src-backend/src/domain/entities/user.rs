use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        deleted_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id: 0,
            username,
            password,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
