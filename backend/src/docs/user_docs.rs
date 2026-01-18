// ============================================
// 3. src/docs/user_docs.rs
// ============================================
use crate::{controllers, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::user_controller::get_users,
        controllers::user_controller::get_user_by_id,
        controllers::user_controller::get_current_user,
        controllers::user_controller::update_current_user,
        controllers::user_controller::change_password,
        controllers::user_controller::delete_user,
    ),
    components(
        schemas(
            models::user::UserResponse,
            models::user::UserListResponse,
            models::user::UpdateUserRequest,
            models::user::ChangePasswordRequest,
        )
    ),
    tags(
        (name = "users", description = "User management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct UserApiDoc;

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
