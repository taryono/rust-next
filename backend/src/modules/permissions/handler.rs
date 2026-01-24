// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};
/// Create permission
#[utoipa::path(
    post,
    path = "/api/permissions",
    request_body = CreatePermissionRequest,
    responses(
        (status = 201, description = "Permission created successfully", body = PermissionResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Permission "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .permission_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get permission by ID
#[utoipa::path(
    get,
    path = "/api/permissions/{id}",
    params(
        ("id" = i64, Path, description = "Permission ID")
    ),
    responses(
        (status = 200, description = "Permission found", body = PermissionResponse),
        (status = 404, description = "Permission not found")
    ),
    tag = "Permission "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .permission_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all permissions with pagination
#[utoipa::path(
    get,
    path = "/api/permissions",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of permissions", body = PaginatedResponse<PermissionResponse>)
    ),
    tag = "Permission "
)]

pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    match app_state.permission_service.get_all(params, None).await {
        Ok(roles) => Ok(HttpResponse::Ok().json(ApiResponse::success(roles))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())))
        }
    }
}

/// Update permission
#[utoipa::path(
    put,
    path = "/api/permissions/{id}",
    params(
        ("id" = i64, Path, description = "Permission ID")
    ),
    request_body = UpdatePermissionRequest,
    responses(
        (status = 200, description = "Permission updated", body = PermissionResponse),
        (status = 404, description = "Permission not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Permission "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .permission_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete permission
#[utoipa::path(
    delete,
    path = "/api/permissions/{id}",
    params(
        ("id" = i64, Path, description = "Permission ID")
    ),
    responses(
        (status = 204, description = "Permission deleted"),
        (status = 404, description = "Permission not found")
    ),
    tag = "Permission "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.permission_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
