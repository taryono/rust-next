// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateDepartmentRequest, DepartmentResponse, UpdateDepartmentRequest};
use super::service::DepartmentService;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create department
#[utoipa::path(
    post,
    path = "/api/departments",
    request_body = CreateDepartmentRequest,
    responses(
        (status = 201, description = "Department created successfully", body = DepartmentResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Department "
)]
pub async fn create(
    service: web::Data<DepartmentService>,
    request: web::Json<CreateDepartmentRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get department by ID
#[utoipa::path(
    get,
    path = "/api/departments/{id}",
    params(
        ("id" = i64, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department found", body = DepartmentResponse),
        (status = 404, description = "Department not found")
    ),
    tag = "Department "
)]
pub async fn get_by_id(
    service: web::Data<DepartmentService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all departments with pagination
#[utoipa::path(
    get,
    path = "/api/departments",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of departments", body = PaginatedResponse<DepartmentResponse>)
    ),
    tag = "Department "
)]
pub async fn get_all(
    service: web::Data<DepartmentService>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update department
#[utoipa::path(
    put,
    path = "/api/departments/{id}",
    params(
        ("id" = i64, Path, description = "Department ID")
    ),
    request_body = UpdateDepartmentRequest,
    responses(
        (status = 200, description = "Department updated", body = DepartmentResponse),
        (status = 404, description = "Department not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Department "
)]
pub async fn update(
    service: web::Data<DepartmentService>,
    id: web::Path<i64>,
    request: web::Json<UpdateDepartmentRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete department
#[utoipa::path(
    delete,
    path = "/api/departments/{id}",
    params(
        ("id" = i64, Path, description = "Department ID")
    ),
    responses(
        (status = 204, description = "Department deleted"),
        (status = 404, description = "Department not found")
    ),
    tag = "Department "
)]
pub async fn delete(
    service: web::Data<DepartmentService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
