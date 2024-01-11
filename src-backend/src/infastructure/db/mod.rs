use once_cell::sync::Lazy;
use sqlx::{MySqlPool, PgPool};
use std::sync::Mutex;

pub mod todo;
pub mod user;

pub enum Database {
    MySQL(MySqlPool),
    PgSQL(PgPool),
}

pub static DB: Lazy<Mutex<Option<Database>>> = Lazy::new(|| Mutex::new(None));

pub async fn init_db() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_type = std::env::var("DATABASE_TYPE").expect("DATABASE_TYPE must be set");
    let database = match db_type.as_str() {
        "mysql" => {
            let pool = MySqlPool::connect(&db_url).await.unwrap();
            Database::MySQL(pool)
        }
        "postgresql" => {
            let pool = PgPool::connect(&db_url).await.unwrap();
            Database::PgSQL(pool)
        }
        _ => panic!("Invalid database type"),
    };
    let mut db = DB.lock().unwrap();
    *db = Some(database);
}
