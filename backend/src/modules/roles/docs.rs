// src/docs/role_docs.rs
use crate::modules::roles::{dto, role};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        role::get_roles,
        role::get_role_by_id,
        role::get_current_role,
        role::update_current_role,
        role::delete_role,
        role::restore_role,
        role::force_delete_role,
        role::get_deleted_roles,
    ),
    components(
        schemas(
            dto::RoleResponse,
            dto::RoleListResponse,
            dto::UpdateRoleRequest,
        )
    ),
    tags(
        (name = "roles", description = "Role management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct RolesApiDoc;

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
