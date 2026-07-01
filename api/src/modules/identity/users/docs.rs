// ============================================
// src/modules/users/docs.rs
// ============================================
use crate::modules::identity::users::{dto, dto_multipart, handler};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // CRUD
        handler::get_users,
        handler::get_by_id,
        handler::get_current_user,
        handler::create,
        handler::update,
        handler::change_password,
        handler::delete_user,
        handler::restore_user,
        handler::force_delete_user,
        // Role management
        handler::get_user_with_roles,
        handler::assign_role,
        handler::remove_role,
        handler::sync_roles,
        // Multipart
        handler::create_multipart,
    ),
    components(
        schemas(
            // Response
            dto::UserResponse,
            dto::UserListResponse,
            // Request - CRUD
            dto::CreateUserRequest,
            dto::UpdateUserRequest,
            dto::ChangePasswordRequest,
            // Request - Role management
            dto::AssignRoleRequest,
            dto::SyncRolesRequest,
            // Request - Multipart
            dto_multipart::CreateUserMultipartRequest,
        )
    ),
    tags(
        (name = "Users", description = "User management endpoints")
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
