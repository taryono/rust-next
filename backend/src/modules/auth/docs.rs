// ============================================
// 2. src/docs/auth_docs.rs
// ============================================
use crate::modules::auth::{dto, handler};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::register,
        handler::login,
        handler::refresh,
    ),
    components(
        schemas(
            dto::RegisterRequest,
            dto::LoginRequest,
            dto::AuthResponse,
            dto::UserInfo,
            dto::RefreshTokenRequest,
            dto::RefreshTokenResponse,
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
