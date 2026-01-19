// ============================================
// 2. src/docs/auth_docs.rs
// ============================================
use crate::modules::auth::{auth, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        auth::refresh,
    ),
    components(
        schemas(
            models::RegisterRequest,
            models::LoginRequest,
            models::AuthResponse,
            models::UserInfo,
            models::RefreshTokenRequest,
            models::RefreshTokenResponse,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct AuthApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}
