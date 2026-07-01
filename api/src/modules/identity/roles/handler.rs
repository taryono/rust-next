// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateRoleRequest, RoleResponse, UpdateRoleRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};

use actix_web::{web, HttpResponse};
/// Create role
#[utoipa::path(
    post,
    path = "/api/roles",
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created successfully", body = RoleResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Roles"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.role_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get role by ID
#[utoipa::path(
    get,
    path = "/api/roles/{id}",
    params(
        ("id" = i64, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role found", body = RoleResponse),
        (status = 404, description = "Role not found")
    ),
    tag = "Roles"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.role_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all roles with pagination
#[utoipa::path(
    get,
    path = "/api/roles",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of roles", body = PaginatedResponse<RoleResponse>)
    ),
    tag = "Roles"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();

    // Jika pakai multi-tenant by foundation
    // let result = app_state.role_app_state.permission_serviceget_all(params, Some(*foundation_id)).await?;

    // Untuk admin (semua foundation)
    //let result = app_state.role_service.get_all(params, None).await?;

    match app_state.role_service.get_all(params, None).await {
        Ok(roles) => Ok(HttpResponse::Ok().json(ApiResponse::success(roles))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())))
        }
    }
}

/// Update role
#[utoipa::path(
    put,
    path = "/api/roles/{id}",
    params(
        ("id" = i64, Path, description = "Role ID")
    ),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated", body = RoleResponse),
        (status = 404, description = "Role not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Roles"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .role_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete role
#[utoipa::path(
    delete,
    path = "/api/roles/{id}",
    params(
        ("id" = i64, Path, description = "Role ID")
    ),
    responses(
        (status = 204, description = "Role deleted"),
        (status = 404, description = "Role not found")
    ),
    tag = "Roles"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.role_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
