use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at time
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        let exp = now.checked_add_signed(Duration::days(7)).unwrap(); // Token expires in 7 days

        Claims {
            sub: user_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
}

pub fn generate_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
    let claims = Claims::new(user_id);
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

