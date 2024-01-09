use axum::{routing::get, Router};

use super::health_checker;

pub fn create_router() -> Router {
    Router::new().route("/api/health_checker", get(health_checker))
}
