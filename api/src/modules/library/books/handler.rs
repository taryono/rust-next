// ============================================================================
// src/modules/books/handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{BookResponse, CreateBookRequest, UpdateBookRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create student
#[utoipa::path(
    post,
    path = "/api/books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created successfully", body = BookResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Book "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateBookRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.book_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get student by ID
#[utoipa::path(
    get,
    path = "/api/books/{id}",
    params(
        ("id" = i64, Path, description = "Book ID")
    ),
    responses(
        (status = 200, description = "Book found", body = BookResponse),
        (status = 404, description = "Book not found")
    ),
    tag = "Book "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.book_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all students with pagination
#[utoipa::path(
    get,
    path = "/api/books",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of students", body = PaginatedResponse<BookResponse>)
    ),
    tag = "Book "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.book_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update student
#[utoipa::path(
    put,
    path = "/api/books/{id}",
    params(
        ("id" = i64, Path, description = "Book ID")
    ),
    request_body = UpdateBookRequest,
    responses(
        (status = 200, description = "Book updated", body = BookResponse),
        (status = 404, description = "Book not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Book "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateBookRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .book_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete student
#[utoipa::path(
    delete,
    path = "/api/books/{id}",
    params(
        ("id" = i64, Path, description = "Book ID")
    ),
    responses(
        (status = 204, description = "Book deleted"),
        (status = 404, description = "Book not found")
    ),
    tag = "Book "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.book_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
