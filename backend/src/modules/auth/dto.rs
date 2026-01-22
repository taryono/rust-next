use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::modules::users::dto::UserResponse;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}
