// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateFoundationRequest, FoundationResponse, UpdateFoundationRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create foundation
#[utoipa::path(
    post,
    path = "/api/foundations",
    request_body = CreateFoundationRequest,
    responses(
        (status = 201, description = "Foundation created successfully", body = FoundationResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Foundation "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateFoundationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get foundation by ID
#[utoipa::path(
    get,
    path = "/api/foundations/{id}",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 200, description = "Foundation found", body = FoundationResponse),
        (status = 404, description = "Foundation not found")
    ),
    tag = "Foundation "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all foundations with pagination
#[utoipa::path(
    get,
    path = "/api/foundations",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of foundations", body = PaginatedResponse<FoundationResponse>)
    ),
    tag = "Foundation "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.foundation_service.get_all(params).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update foundation
#[utoipa::path(
    put,
    path = "/api/foundations/{id}",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    request_body = UpdateFoundationRequest,
    responses(
        (status = 200, description = "Foundation updated", body = FoundationResponse),
        (status = 404, description = "Foundation not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Foundation "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateFoundationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete foundation
#[utoipa::path(
    delete,
    path = "/api/foundations/{id}",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 204, description = "Foundation deleted"),
        (status = 404, description = "Foundation not found")
    ),
    tag = "Foundation "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.foundation_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
