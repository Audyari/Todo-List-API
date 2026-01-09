use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;

use crate::utils::jwt::Claims;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "No authorization header".to_string()))?;

        let token_data = decode_token(bearer.token())
            .await
            .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(AuthUser {
            user_id: token_data.sub,
        })
    }
}

pub async fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
    
    let mut validation = Validation::default();
    validation.validate_exp = true; // Validate expiration
    
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)
        .map(|data| data.claims)
}