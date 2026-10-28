// ============================================================================
// src/modules/library/authors/handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{AuthorResponse, CreateAuthorRequest, UpdateAuthorRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create author
#[utoipa::path(
    post,
    path = "/api/author",
    request_body = CreateAuthorRequest,
    responses(
        (status = 201, description = "Author created successfully", body = AuthorResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name")
    ),
    tag = "Author"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateAuthorRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.author_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get author by ID
#[utoipa::path(
    get,
    path = "/api/author/{id}",
    params(
        ("id" = i64, Path, description = "Author ID")
    ),
    responses(
        (status = 200, description = "Author found", body = AuthorResponse),
        (status = 404, description = "Author not found")
    ),
    tag = "Author"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.author_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all authors with pagination
#[utoipa::path(
    get,
    path = "/api/author",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of authors", body = PaginatedResponse<AuthorResponse>)
    ),
    tag = "Author"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    let result = app_state.author_service.get_all(params, None).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Update author
#[utoipa::path(
    put,
    path = "/api/author/{id}",
    params(
        ("id" = i64, Path, description = "Author ID")
    ),
    request_body = UpdateAuthorRequest,
    responses(
        (status = 200, description = "Author updated", body = AuthorResponse),
        (status = 404, description = "Author not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Author"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateAuthorRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .author_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete author
#[utoipa::path(
    delete,
    path = "/api/author/{id}",
    params(
        ("id" = i64, Path, description = "Author ID")
    ),
    responses(
        (status = 204, description = "Author deleted"),
        (status = 404, description = "Author not found")
    ),
    tag = "Author"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.author_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
