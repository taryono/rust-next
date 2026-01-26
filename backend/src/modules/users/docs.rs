// ============================================
// 3. src/docs/user_docs.rs
// ============================================
use crate::modules::users::{dto, handler};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::get_users,
        handler::get_by_id,
        handler::get_current_user,
        handler::update_user,
        handler::change_password,
        handler::delete_user,
    ),
    components(
        schemas(
            dto::UserResponse,
            dto::UserListResponse,
            dto::UpdateUserRequest,
            dto::ChangePasswordRequest,
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct UsersApiDoc;

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
