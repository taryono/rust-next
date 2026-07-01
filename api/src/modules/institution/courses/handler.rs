// api/src/modules/institution/course/handler.rs
// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CourseResponse, CreateCourseRequest, UpdateCourseRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create Course
#[utoipa::path(
    post,
    path = "/api/courses",
    request_body = CreateCourseRequest,
    responses(
        (status = 201, description = "Course created successfully", body = CourseResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Course "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .course_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get Course by ID
#[utoipa::path(
    get,
    path = "/api/courses/{id}",
    params(
        ("id" = i64, Path, description = "Course ID")
    ),
    responses(
        (status = 200, description = "Course found", body = CourseResponse),
        (status = 404, description = "Course not found")
    ),
    tag = "Course "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.course_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all Courses with pagination
#[utoipa::path(
    get,
    path = "/api/courses",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of Coursess", body = PaginatedResponse<CourseResponse>)
    ),
    tag = "Course "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.course_service.get_all(params).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update Course
#[utoipa::path(
    put,
    path = "/api/courses/{id}",
    params(
        ("id" = i64, Path, description = "Course ID")
    ),
    request_body = UpdateCourseRequest,
    responses(
        (status = 200, description = "Course updated", body = CourseResponse),
        (status = 404, description = "Course not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Course "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .course_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete Course
#[utoipa::path(
    delete,
    path = "/api/courses/{id}",
    params(
        ("id" = i64, Path, description = "Course ID")
    ),
    responses(
        (status = 204, description = "Course deleted"),
        (status = 404, description = "Course not found")
    ),
    tag = "Course "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.course_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
