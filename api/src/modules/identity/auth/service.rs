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

    pub async fn login(&self, body: LoginRequest) -> Result<AuthResponse, AppError> {
        // 1. Fetch user dengan roles (1x query)
        let (user, roles) = self
            .repository
            .find_by_email_with_roles(&body.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid credentials".into()))?;

        // 2. Verify password
        if !password::verify(&body.password, &user.password)? {
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

        let array_roles: Vec<String> = roles.iter().map(|r| r.code.clone()).collect();

        // 4. Buat tokens
        let (access_token, refresh_token) =
            self.generate_token_pair(user.id, user.foundation_id, &array_roles, &permissions)?;

        Ok(AuthResponse {
            user: UserResponse::from_user_with_roles(&user, &roles),
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.get_token_expiration(),
        })
    }

    pub async fn refresh_token(&self, token: String) -> Result<RefreshTokenResponse, AppError> {
        // 1. Verify refresh token
        let claims = jwt::verify_refresh_token(&token)
            .map_err(|_| AppError::Unauthorized("Invalid or expired refresh token".into()))?;

        // 2. Pastikan ini memang refresh token
        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized("Invalid token type".into()));
        }

        // 3. Cek user masih ada di DB
        let (user, roles) = self
            .repository
            .find_by_id_with_roles(claims.user_id)
            .await?
            .ok_or(AppError::Unauthorized("User not found".into()))?;

        // 4. Re-resolve permissions (bisa berubah sejak token dibuat)
        let permissions = PermissionService::resolve_user_permissions(
            self.repository.conn(),
            user.id,
            user.foundation_id,
        )
        .await?
        .into_iter()
        .collect::<Vec<_>>();

        let array_roles: Vec<String> = roles.iter().map(|r| r.code.clone()).collect();

        // 5. Generate access token baru saja (refresh token tetap yang lama)
        let access_claims = jwt::Claims::new(
            user.id,
            user.foundation_id,
            "access".into(),
            array_roles.clone(),
            permissions,
        );
        let new_refresh_claims = jwt::Claims::new(
            claims.user_id,
            claims.foundation_id,
            "refresh".into(),
            array_roles.clone(),
            claims.permissions,
        );

        let access_token = jwt::create_token(&access_claims)?;
        let refresh_token = jwt::create_refresh_token(&new_refresh_claims)?;
        Ok(RefreshTokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".into(),
            expires_in: self.get_token_expiration(),
        })
    }

    // ← Helper agar tidak duplikasi logic token generation
    fn generate_token_pair(
        &self,
        user_id: i64,
        foundation_id: i64,
        roles: &[String],
        permissions: &[String],
    ) -> Result<(String, String), AppError> {
        let access_claims = jwt::Claims::new(
            user_id,
            foundation_id,
            "access".into(),
            roles.to_vec(),
            permissions.to_vec(),
        );
        let refresh_claims = jwt::Claims::new(
            user_id,
            foundation_id,
            "refresh".into(),
            roles.to_vec(),
            permissions.to_vec(),
        );

        let access_token = jwt::create_token(&access_claims)?;
        let refresh_token = jwt::create_refresh_token(&refresh_claims)?;

        Ok((access_token, refresh_token))
    }

    fn get_token_expiration(&self) -> i64 {
        env::var("JWT_EXPIRATION")
            .unwrap_or("900".to_string())
            .parse()
            .unwrap_or(900)
    }
}
