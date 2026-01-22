// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateSemesterRequest, SemesterResponse, UpdateSemesterRequest};
use super::service::SemesterService;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create semester
#[utoipa::path(
    post,
    path = "/api/semesters",
    request_body = CreateSemesterRequest,
    responses(
        (status = 201, description = "Semester created successfully", body = SemesterResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Semester "
)]
pub async fn create(
    service: web::Data<SemesterService>,
    request: web::Json<CreateSemesterRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get semester by ID
#[utoipa::path(
    get,
    path = "/api/semesters/{id}",
    params(
        ("id" = i64, Path, description = "Semester ID")
    ),
    responses(
        (status = 200, description = "Semester found", body = SemesterResponse),
        (status = 404, description = "Semester not found")
    ),
    tag = "Semester "
)]
pub async fn get_by_id(
    service: web::Data<SemesterService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all semesters with pagination
#[utoipa::path(
    get,
    path = "/api/semesters",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of semesters", body = PaginatedResponse<SemesterResponse>)
    ),
    tag = "Semester "
)]
pub async fn get_all(
    service: web::Data<SemesterService>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update semester
#[utoipa::path(
    put,
    path = "/api/semesters/{id}",
    params(
        ("id" = i64, Path, description = "Semester ID")
    ),
    request_body = UpdateSemesterRequest,
    responses(
        (status = 200, description = "Semester updated", body = SemesterResponse),
        (status = 404, description = "Semester not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Semester "
)]
pub async fn update(
    service: web::Data<SemesterService>,
    id: web::Path<i64>,
    request: web::Json<UpdateSemesterRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete semester
#[utoipa::path(
    delete,
    path = "/api/semesters/{id}",
    params(
        ("id" = i64, Path, description = "Semester ID")
    ),
    responses(
        (status = 204, description = "Semester deleted"),
        (status = 404, description = "Semester not found")
    ),
    tag = "Semester "
)]
pub async fn delete(
    service: web::Data<SemesterService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
