// ============================================
// backend/src/controllers/role_controller.rs
// ============================================
use crate::utils::pagination::PaginationParams;
use crate::{
    config::database::Database,
    modules::roles::models::{RoleListResponse, RoleResponse, UpdateRoleRequest},
    modules::roles::services,
    utils::{jwt::Claims, response::ApiResponse},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/roles",
    tag = "roles",
    params(
        ("page" = Option<u64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<u64>, Query, description = "Items per page, default 10, max 100"),
        ("search" = Option<String>, Query, description = "Search by name or description"),
    ),
    responses(
        (status = 200, description = "List of roles retrieved successfully", body = RoleListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_roles(
    db: web::Data<Database>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    match services::get_all_roles(db.get_connection(), query.into_inner()).await {
        Ok(roles) => HttpResponse::Ok().json(ApiResponse::success(roles)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/{id}",
    tag = "roles",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role found", body = RoleResponse),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_role_by_id(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    let role_id = path.into_inner();

    match services::get_role_by_id(db.get_connection(), role_id).await {
        Ok(role) => HttpResponse::Ok().json(ApiResponse::success(role)),
        Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/roles/me",
    tag = "roles",
    responses(
        (status = 200, description = "Current role profile", body = RoleResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Role not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_role(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let role_id: u64 = claims.sub.parse().unwrap_or(0);

            match services::get_role_by_id(db.get_connection(), role_id).await {
                Ok(role) => HttpResponse::Ok().json(ApiResponse::success(role)),
                Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string())),
            }
        }
        None => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Unauthorized".to_string()))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/roles/me",
    tag = "roles",
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated successfully", body = RoleResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_role(
    db: web::Data<Database>,
    req: HttpRequest,
    body: web::Json<UpdateRoleRequest>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!(
            "Validation error: {}",
            errors
        )));
    }

    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let role_id: u64 = claims.sub.parse().unwrap_or(0);

            match services::update_role(db.get_connection(), role_id, body.into_inner()).await {
                Ok(role) => HttpResponse::Ok().json(ApiResponse::success(role)),
                Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
            }
        }
        None => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Unauthorized".to_string()))
        }
    }
}

/// Soft delete role (replaces delete_role)
#[utoipa::path(
    delete,
    path = "/api/roles/{id}",
    tag = "roles",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role soft deleted successfully"),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_role(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    let role_id = path.into_inner();

    match services::soft_delete(db.get_connection(), role_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Role deleted successfully")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Restore deleted role
#[utoipa::path(
    post,
    path = "/api/roles/{id}/restore",
    tag = "roles",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role restored successfully"),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_role(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    let role_id = path.into_inner();

    match services::restore(db.get_connection(), role_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Role restored successfully")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Force delete role (permanent)
#[utoipa::path(
    delete,
    path = "/api/roles/{id}/force",
    tag = "roles",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role permanently deleted"),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn force_delete_role(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    let role_id = path.into_inner();

    match services::force_delete(db.get_connection(), role_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Role permanently deleted")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get deleted roles
#[utoipa::path(
    get,
    path = "/api/roles/deleted",
    tag = "roles",
    params(
        ("page" = Option<u64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<u64>, Query, description = "Items per page, default 10, max 100"),
    ),
    responses(
        (status = 200, description = "List of deleted roles"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_deleted_roles(
    db: web::Data<Database>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    match services::get_deleted_roles(db.get_connection(), query.into_inner()).await {
        Ok(roles) => HttpResponse::Ok().json(ApiResponse::success(roles)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}
