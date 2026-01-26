// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateStudentRequest, StudentResponse, UpdateStudentRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create student
#[utoipa::path(
    post,
    path = "/api/students",
    request_body = CreateStudentRequest,
    responses(
        (status = 201, description = "Student created successfully", body = StudentResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Student "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateStudentRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .student_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get student by ID
#[utoipa::path(
    get,
    path = "/api/students/{id}",
    params(
        ("id" = i64, Path, description = "Student ID")
    ),
    responses(
        (status = 200, description = "Student found", body = StudentResponse),
        (status = 404, description = "Student not found")
    ),
    tag = "Student "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.student_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all students with pagination
#[utoipa::path(
    get,
    path = "/api/students",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of students", body = PaginatedResponse<StudentResponse>)
    ),
    tag = "Student "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.student_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update student
#[utoipa::path(
    put,
    path = "/api/students/{id}",
    params(
        ("id" = i64, Path, description = "Student ID")
    ),
    request_body = UpdateStudentRequest,
    responses(
        (status = 200, description = "Student updated", body = StudentResponse),
        (status = 404, description = "Student not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Student "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateStudentRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .student_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete student
#[utoipa::path(
    delete,
    path = "/api/students/{id}",
    params(
        ("id" = i64, Path, description = "Student ID")
    ),
    responses(
        (status = 204, description = "Student deleted"),
        (status = 404, description = "Student not found")
    ),
    tag = "Student "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.student_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
