use anyhow::Result;
use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::api::{
    self,
    request::{error_response, Response},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: i32) -> Result<String> {
    let expiration = Local::now() + Duration::hours(24);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

// 包装成一个中间件
pub struct JwtMiddleware;

#[async_trait::async_trait]
impl<B> FromRequestParts<B> for JwtMiddleware
where
    B: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _body: &B) -> Result<Self, Self::Rejection> {
        let mut headers = parts.headers.clone();
        let token = headers
            .remove("Authorization")
            .ok_or_else(|| error_response(4001, "Invalid token".to_string()))?;
        let token = token
            .to_str()
            .map_err(|_| error_response(4001, "Invalid token".to_string()))?;
        let token = token.replace("Bearer ", "");
        let claims =
            verify_token(&token).map_err(|_| error_response(4001, "Invalid token".to_string()))?;
        let now = Local::now().timestamp() as usize;
        if now > claims.exp {
            return Err(error_response(4001, "Invalid token".to_string()));
        }

        claims
            .sub
            .parse::<i32>()
            .map_err(|_| error_response(4001, "Invalid token".to_string()))?;
        // 在上下文写入用户id
        parts.extensions.insert(claims.sub.parse::<i32>().unwrap());
        Ok(JwtMiddleware)
    }
}
