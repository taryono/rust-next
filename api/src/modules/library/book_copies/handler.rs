// ============================================================================
// src/modules/book_copies/handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{BookCopiesResponse, CreateBookCopiesRequest, UpdateBookCopiesRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create student
#[utoipa::path(
    post,
    path = "/api/book_copies",
    request_body = CreateBookCopiesRequest,
    responses(
        (status = 201, description = "BookCopies created successfully", body = BookCopiesResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "BookCopies "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateBookCopiesRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .book_copy_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get student by ID
#[utoipa::path(
    get,
    path = "/api/book_copies/{id}",
    params(
        ("id" = i64, Path, description = "BookCopies ID")
    ),
    responses(
        (status = 200, description = "BookCopies found", body = BookCopiesResponse),
        (status = 404, description = "BookCopies not found")
    ),
    tag = "BookCopies "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .book_copy_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all students with pagination
#[utoipa::path(
    get,
    path = "/api/book_copies",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of students", body = PaginatedResponse<BookCopiesResponse>)
    ),
    tag = "BookCopies "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.book_copy_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update student
#[utoipa::path(
    put,
    path = "/api/book_copies/{id}",
    params(
        ("id" = i64, Path, description = "BookCopies ID")
    ),
    request_body = UpdateBookCopiesRequest,
    responses(
        (status = 200, description = "BookCopies updated", body = BookCopiesResponse),
        (status = 404, description = "BookCopies not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "BookCopies "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateBookCopiesRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .book_copy_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete student
#[utoipa::path(
    delete,
    path = "/api/book_copies/{id}",
    params(
        ("id" = i64, Path, description = "BookCopies ID")
    ),
    responses(
        (status = 204, description = "BookCopies deleted"),
        (status = 404, description = "BookCopies not found")
    ),
    tag = "BookCopies "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.book_copy_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
