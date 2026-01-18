// src/docs/role_docs.rs
use crate::{controllers, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::role_controller::get_roles,
        controllers::role_controller::get_role_by_id,
        controllers::role_controller::get_current_role,
        controllers::role_controller::update_current_role,
        controllers::role_controller::delete_role,
        controllers::role_controller::restore_role,
        controllers::role_controller::force_delete_role,
        controllers::role_controller::get_deleted_roles,
    ),
    components(
        schemas(
            models::role::RoleResponse,
            models::role::RoleListResponse,
            models::role::UpdateRoleRequest,
        )
    ),
    tags(
        (name = "roles", description = "Role management endpoints")
    )
)]
pub struct RoleApiDoc;
