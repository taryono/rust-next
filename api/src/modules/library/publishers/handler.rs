// ============================================================================
// src/modules/library/publishers/handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreatePublisherRequest, PublisherResponse, UpdatePublisherRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create publisher
#[utoipa::path(
    post,
    path = "/api/publisher",
    request_body = CreatePublisherRequest,
    responses(
        (status = 201, description = "Publisher created successfully", body = PublisherResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name")
    ),
    tag = "Publisher"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreatePublisherRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.publisher_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get publisher by ID
#[utoipa::path(
    get,
    path = "/api/publisher/{id}",
    params(
        ("id" = i64, Path, description = "Publisher ID")
    ),
    responses(
        (status = 200, description = "Publisher found", body = PublisherResponse),
        (status = 404, description = "Publisher not found")
    ),
    tag = "Publisher"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.publisher_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all publishers with pagination
#[utoipa::path(
    get,
    path = "/api/publisher",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of publishers", body = PaginatedResponse<PublisherResponse>)
    ),
    tag = "Publisher"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    let result = app_state.publisher_service.get_all(params, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Update publisher
#[utoipa::path(
    put,
    path = "/api/publisher/{id}",
    params(
        ("id" = i64, Path, description = "Publisher ID")
    ),
    request_body = UpdatePublisherRequest,
    responses(
        (status = 200, description = "Publisher updated", body = PublisherResponse),
        (status = 404, description = "Publisher not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Publisher"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdatePublisherRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .publisher_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete publisher
#[utoipa::path(
    delete,
    path = "/api/publisher/{id}",
    params(
        ("id" = i64, Path, description = "Publisher ID")
    ),
    responses(
        (status = 204, description = "Publisher deleted"),
        (status = 404, description = "Publisher not found")
    ),
    tag = "Publisher"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.publisher_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
