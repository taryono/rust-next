// api/src/modules/institution/sport/handler.rs
// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateSportRequest, SportResponse, UpdateSportRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create Sport
#[utoipa::path(
    post,
    path = "/api/sports",
    request_body = CreateSportRequest,
    responses(
        (status = 201, description = "Sport created successfully", body = SportResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Sport "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateSportRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_type_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get Sport by ID
#[utoipa::path(
    get,
    path = "/api/sports/{id}",
    params(
        ("id" = i64, Path, description = "Sport ID")
    ),
    responses(
        (status = 200, description = "Sport found", body = SportResponse),
        (status = 404, description = "Sport not found")
    ),
    tag = "Sport "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_type_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all Sports with pagination
#[utoipa::path(
    get,
    path = "/api/sports",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of Foundation Types", body = PaginatedResponse<SportResponse>)
    ),
    tag = "Sport "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.foundation_type_service.get_all(params).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update Sport
#[utoipa::path(
    put,
    path = "/api/sports/{id}",
    params(
        ("id" = i64, Path, description = "Sport ID")
    ),
    request_body = UpdateSportRequest,
    responses(
        (status = 200, description = "Sport updated", body = SportResponse),
        (status = 404, description = "Sport not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Sport "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateSportRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_type_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete Sport
#[utoipa::path(
    delete,
    path = "/api/sports/{id}",
    params(
        ("id" = i64, Path, description = "Sport ID")
    ),
    responses(
        (status = 204, description = "Sport deleted"),
        (status = 404, description = "Sport not found")
    ),
    tag = "Sport "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state
        .foundation_type_service
        .delete(id.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
