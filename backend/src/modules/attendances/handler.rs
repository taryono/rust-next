// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{AttendanceResponse, CreateAttendanceRequest, UpdateAttendanceRequest};
use super::service::AttendanceService;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create attendance
#[utoipa::path(
    post,
    path = "/api/attendances",
    request_body = CreateAttendanceRequest,
    responses(
        (status = 201, description = "Attendance created successfully", body = AttendanceResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Attendance "
)]
pub async fn create(
    service: web::Data<AttendanceService>,
    request: web::Json<CreateAttendanceRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get attendance by ID
#[utoipa::path(
    get,
    path = "/api/attendances/{id}",
    params(
        ("id" = i64, Path, description = "Attendance ID")
    ),
    responses(
        (status = 200, description = "Attendance found", body = AttendanceResponse),
        (status = 404, description = "Attendance not found")
    ),
    tag = "Attendance "
)]
pub async fn get_by_id(
    service: web::Data<AttendanceService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all attendances with pagination
#[utoipa::path(
    get,
    path = "/api/attendances",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of attendances", body = PaginatedResponse<AttendanceResponse>)
    ),
    tag = "Attendance "
)]
pub async fn get_all(
    service: web::Data<AttendanceService>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = service.get_all(params, foundation_id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update attendance
#[utoipa::path(
    put,
    path = "/api/attendances/{id}",
    params(
        ("id" = i64, Path, description = "Attendance ID")
    ),
    request_body = UpdateAttendanceRequest,
    responses(
        (status = 200, description = "Attendance updated", body = AttendanceResponse),
        (status = 404, description = "Attendance not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Attendance "
)]
pub async fn update(
    service: web::Data<AttendanceService>,
    id: web::Path<i64>,
    request: web::Json<UpdateAttendanceRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete attendance
#[utoipa::path(
    delete,
    path = "/api/attendances/{id}",
    params(
        ("id" = i64, Path, description = "Attendance ID")
    ),
    responses(
        (status = 204, description = "Attendance deleted"),
        (status = 404, description = "Attendance not found")
    ),
    tag = "Attendance "
)]
pub async fn delete(
    service: web::Data<AttendanceService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
