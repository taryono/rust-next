// ============================================
// 2. src/docs/auth_docs.rs
// ============================================
use crate::{controllers, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::auth_controller::register,
        controllers::auth_controller::login,
        controllers::auth_controller::refresh,
    ),
    components(
        schemas(
            models::auth::RegisterRequest,
            models::auth::LoginRequest,
            models::auth::AuthResponse,
            models::auth::UserInfo,
            models::auth::RefreshTokenRequest,
            models::auth::RefreshTokenResponse,
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
