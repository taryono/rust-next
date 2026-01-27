// src/modules/auth/service.rs
use crate::modules::permissions::PermissionService;
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
            return Err(AppError::BadRequest("Email already exists".into()));
        }

        let user = self
            .repository
            .create_user(body.name, body.email, body.password)
            .await?;

        Ok(UserResponse::from_entity(&user))
    }

    // ✅ FIX: Login method yang benar
    pub async fn login(&self, body: LoginRequest) -> Result<AuthResponse, AppError> {
        // 1. Fetch user dengan roles sekalian (hanya 1x query)
        let (user, roles) = self
            .repository
            .find_by_email_with_roles(&body.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

        // 2. Verify password
        let valid = password::verify(&body.password, &user.password)?;
        if !valid {
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }

        // 3. Resolve permissions
        let permissions = PermissionService::resolve_user_permissions(
            self.repository.conn(),
            user.id,
            user.foundation_id,
        )
        .await?
        .into_iter()
        .collect::<Vec<_>>();

        // 4. Create JWT claims dengan permissions
        let access_claims =
            jwt::Claims::new(user.id.to_string(), "access".into(), permissions.clone());

        let refresh_claims = jwt::Claims::new(user.id.to_string(), "refresh".into(), permissions);

        // 5. Generate tokens (TANPA .await karena bukan async function)
        let access_token = jwt::create_token(&access_claims)?;
        let refresh_token = jwt::create_refresh_token(&refresh_claims)?;

        // 6. Return response
        Ok(AuthResponse {
            user: UserResponse::from_user_with_roles(&user, &roles),
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.get_token_expiration(),
        })
    }

    pub async fn refresh_token(&self, token: String) -> Result<RefreshTokenResponse, AppError> {
        // Verify refresh token
        let claims = jwt::verify_refresh_token(&token)?;

        // Create new claims dengan permissions yang sama
        let new_access_claims = jwt::Claims::new(
            claims.sub.clone(),
            "access".into(),
            claims.permissions.clone(),
        );

        let new_refresh_claims = jwt::Claims::new(claims.sub, "refresh".into(), claims.permissions);

        // Generate new tokens (TANPA .await)
        let access_token = jwt::create_token(&new_access_claims)?;
        let refresh_token = jwt::create_refresh_token(&new_refresh_claims)?;

        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.get_token_expiration(),
        })
    }

    fn get_token_expiration(&self) -> i64 {
        env::var("JWT_EXPIRATION")
            .unwrap_or("900".to_string())
            .parse()
            .unwrap_or(900)
    }
}
