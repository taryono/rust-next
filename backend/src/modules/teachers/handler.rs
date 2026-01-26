// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateTeacherRequest, TeacherResponse, UpdateTeacherRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create teacher
#[utoipa::path(
    post,
    path = "/api/teachers",
    request_body = CreateTeacherRequest,
    responses(
        (status = 201, description = "Teacher created successfully", body = TeacherResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Teacher "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateTeacherRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .teacher_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get teacher by ID
#[utoipa::path(
    get,
    path = "/api/teachers/{id}",
    params(
        ("id" = i64, Path, description = "Teacher ID")
    ),
    responses(
        (status = 200, description = "Teacher found", body = TeacherResponse),
        (status = 404, description = "Teacher not found")
    ),
    tag = "Teacher "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.teacher_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all teachers with pagination
#[utoipa::path(
    get,
    path = "/api/teachers",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of teachers", body = PaginatedResponse<TeacherResponse>)
    ),
    tag = "Teacher "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.teacher_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update teacher
#[utoipa::path(
    put,
    path = "/api/teachers/{id}",
    params(
        ("id" = i64, Path, description = "Teacher ID")
    ),
    request_body = UpdateTeacherRequest,
    responses(
        (status = 200, description = "Teacher updated", body = TeacherResponse),
        (status = 404, description = "Teacher not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Teacher "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateTeacherRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .teacher_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete teacher
#[utoipa::path(
    delete,
    path = "/api/teachers/{id}",
    params(
        ("id" = i64, Path, description = "Teacher ID")
    ),
    responses(
        (status = 204, description = "Teacher deleted"),
        (status = 404, description = "Teacher not found")
    ),
    tag = "Teacher "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.teacher_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
