// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateUniversityRequest, UniversityResponse, UpdateUniversityRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create University
#[utoipa::path(
    post,
    path = "/api/universities",
    request_body = CreateUniversityRequest,
    responses(
        (status = 201, description = "University created successfully", body = UniversityResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "University "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateUniversityRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .university_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get University by ID
#[utoipa::path(
    get,
    path = "/api/universities/{id}",
    params(
        ("id" = i64, Path, description = "University ID")
    ),
    responses(
        (status = 200, description = "University found", body = UniversityResponse),
        (status = 404, description = "University not found")
    ),
    tag = "University "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .university_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all Universitys with pagination
#[utoipa::path(
    get,
    path = "/api/universities",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of Universitys", body = PaginatedResponse<UniversityResponse>)
    ),
    tag = "University "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.university_service.get_all(params).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update University
#[utoipa::path(
    put,
    path = "/api/universities/{id}",
    params(
        ("id" = i64, Path, description = "University ID")
    ),
    request_body = UpdateUniversityRequest,
    responses(
        (status = 200, description = "University updated", body = UniversityResponse),
        (status = 404, description = "University not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "University "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateUniversityRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .university_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete University
#[utoipa::path(
    delete,
    path = "/api/universities/{id}",
    params(
        ("id" = i64, Path, description = "University ID")
    ),
    responses(
        (status = 204, description = "University deleted"),
        (status = 404, description = "University not found")
    ),
    tag = "University "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.university_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
