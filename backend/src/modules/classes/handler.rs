// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{ClassResponse, CreateClassRequest, UpdateClassRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create class
#[utoipa::path(
    post,
    path = "/api/classes",
    request_body = CreateClassRequest,
    responses(
        (status = 201, description = "Class created successfully", body = ClassResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Class "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateClassRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.class_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get class by ID
#[utoipa::path(
    get,
    path = "/api/classes/{id}",
    params(
        ("id" = i64, Path, description = "Class ID")
    ),
    responses(
        (status = 200, description = "Class found", body = ClassResponse),
        (status = 404, description = "Class not found")
    ),
    tag = "Class "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.class_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all classes with pagination
#[utoipa::path(
    get,
    path = "/api/classes",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of classes", body = PaginatedResponse<ClassResponse>)
    ),
    tag = "Class "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.class_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update class
#[utoipa::path(
    put,
    path = "/api/classes/{id}",
    params(
        ("id" = i64, Path, description = "Class ID")
    ),
    request_body = UpdateClassRequest,
    responses(
        (status = 200, description = "Class updated", body = ClassResponse),
        (status = 404, description = "Class not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Class "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateClassRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .class_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete class
#[utoipa::path(
    delete,
    path = "/api/classes/{id}",
    params(
        ("id" = i64, Path, description = "Class ID")
    ),
    responses(
        (status = 204, description = "Class deleted"),
        (status = 404, description = "Class not found")
    ),
    tag = "Class "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.class_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
