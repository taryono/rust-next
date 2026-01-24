// src/modules/auth/service.rs
use crate::{
    errors::AppError,
    modules::auth::{
        dto::{AuthResponse, LoginRequest, RefreshTokenResponse, RegisterRequest},
        repository::AuthRepository,
    },
    modules::users::dto::UserResponse,
    utils::{jwt, password},
};
use std::env;

#[derive(Clone)]
pub struct AuthService {
    repository: AuthRepository,
}
impl AuthService {
    pub fn new(repository: AuthRepository) -> Self {
        Self { repository }
    }

    pub async fn register(&self, body: RegisterRequest) -> Result<UserResponse, AppError> {
        let existing_user = self.repository.find_by_email(&body.email).await?;

        if existing_user.is_some() {
            return Err(AppError::NotFoundError("Email already exists".into()));
        }

        let user = self
            .repository
            .create_user(body.name, body.email, body.password)
            .await?;

        Ok(UserResponse::from_entity(&user))
    }

    pub async fn login(&self, body: LoginRequest) -> Result<AuthResponse, AppError> {
        let user = self
            .repository
            .find_by_email(&body.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

        let valid = password::verify(&body.password, &user.password)?;
        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        let access_token = jwt::create_token(user.id.to_string())?;
        let refresh_token = jwt::create_refresh_token(user.id.to_string())?;

        let (user, roles) = self
            .repository
            .find_by_email_with_roles(&body.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

        Ok(AuthResponse {
            user: UserResponse::from_user_with_roles(&user, &roles),
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.get_token_expiration(),
        })
    }

    pub async fn refresh_token(&self, token: String) -> Result<RefreshTokenResponse, AppError> {
        let claims = jwt::verify_refresh_token(&token)?;

        let user_id = claims.sub.clone();

        let access_token = jwt::create_token(user_id.clone())?;
        let refresh_token = jwt::create_refresh_token(user_id)?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.get_token_expiration(),
        })
    }

    fn get_token_expiration(&self) -> i64 {
        let expiration = env::var("JWT_EXPIRATION").unwrap_or("3600".to_string());
        expiration.parse().unwrap()
    }
}
