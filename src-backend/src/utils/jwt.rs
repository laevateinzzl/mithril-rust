use anyhow::Result;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
