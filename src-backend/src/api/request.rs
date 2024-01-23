/*
 * @Author: zzl
 * @Date: 2024-01-11 10:20:47
 * @LastEditTime: 2024-01-11 10:43:10
 * @LastEditors: zzl
 * @Description: 定义通用请求和响应结构体
 * @FilePath: /src-backend/src/api/request.rs
 */

use axum::{extract::FromRequestParts, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
}

pub fn default_pagination() -> Pagination {
    Pagination {
        page: 1,
        page_size: 10,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sort {
    pub field: String,
    pub order: String,
}

pub fn default_sort(key: String) -> Sort {
    Sort {
        field: key,
        order: "desc".to_string(),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let json_response = serde_json::json!({
            "code":self.code,
            "message":self.message,
            "data":Some(self.data)
        });
        axum::Json(json_response).into_response()
    }
}

pub fn success_response(data: serde_json::Value) -> Response {
    Response {
        code: 200,
        message: "success".to_string(),
        data: Some(data),
    }
}

pub fn error_response(code: i32, message: String) -> Response {
    Response {
        code,
        message,
        data: None,
    }
}
