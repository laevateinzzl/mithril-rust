use axum::{response::IntoResponse, Json};

pub mod middleware;
pub mod request;
pub mod router;
pub mod todo;
pub mod user;

pub async fn health_checker() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine";
    let json_response = serde_json::json!({
        "status":"success",
        "message":MESSAGE
    });
    Json(json_response)
}
