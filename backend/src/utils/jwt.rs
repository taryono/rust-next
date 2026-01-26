// backend/src/modules/auth/service.rs
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user id)
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub token_type: String, // "access" or "refresh"
    pub permissions: Vec<String>,
}

impl Claims {
    pub fn new(user_id: String, token_type: String, permissions: Vec<String>) -> Self {
        let expiration = if token_type == "refresh" {
            // Refresh token: 7 days
            env::var("JWT_REFRESH_EXPIRATION")
                .unwrap_or_else(|_| "604800".to_string())
                .parse::<i64>()
                .unwrap_or(604800)
        } else {
            // Access token: 15 minutes
            env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "900".to_string())
                .parse::<i64>()
                .unwrap_or(900)
        };

        let iat = Utc::now();
        let exp = iat + Duration::seconds(expiration);

        Self {
            sub: user_id,
            iat: iat.timestamp() as usize,
            exp: exp.timestamp() as usize,
            token_type,
            permissions,
        }
    }
}

// pub async fn create_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
//     let claims = Claims::new(user_id, "access".to_string());
//     let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     let permissions = PermissionService::resolve_user_permissions(db, user.id).await?;
//     claims.permissions = permissions.into_iter().collect();
//     encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(secret.as_bytes()),
//     )
// }

pub fn create_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn create_refresh_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_REFRESH_SECRET")
        .unwrap_or_else(|_| env::var("JWT_SECRET").expect("JWT_SECRET must be set"));

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_REFRESH_SECRET")
        .unwrap_or_else(|_| env::var("JWT_SECRET").expect("JWT_SECRET must be set"));

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    // Verify it's a refresh token
    if token_data.claims.token_type != "refresh" {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(token_data.claims)
}
