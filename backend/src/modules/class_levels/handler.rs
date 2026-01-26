// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{ClassLevelResponse, CreateClassLevelRequest, UpdateClassLevelRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create class_level
#[utoipa::path(
    post,
    path = "/api/class_levels",
    request_body = CreateClassLevelRequest,
    responses(
        (status = 201, description = "Class Levelcreated successfully", body = ClassLevelResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Class Level"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateClassLevelRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .class_level_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get class_level by ID
#[utoipa::path(
    get,
    path = "/api/class_levels/{id}",
    params(
        ("id" = i64, Path, description = "Class Level ID")
    ),
    responses(
        (status = 200, description = "Class Level found", body = ClassLevelResponse),
        (status = 404, description = "Class Level not found")
    ),
    tag = "Class Level"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .class_level_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all class_levels with pagination
#[utoipa::path(
    get,
    path = "/api/class_levels",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of class_levels", body = PaginatedResponse<ClassLevelResponse>)
    ),
    tag = "Class Level"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.class_level_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update class_level
#[utoipa::path(
    put,
    path = "/api/class_levels/{id}",
    params(
        ("id" = i64, Path, description = "Class Level ID")
    ),
    request_body = UpdateClassLevelRequest,
    responses(
        (status = 200, description = "Class Levelupdated", body = ClassLevelResponse),
        (status = 404, description = "Class Level not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Class Level"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateClassLevelRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .class_level_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete class_level
#[utoipa::path(
    delete,
    path = "/api/class_levels/{id}",
    params(
        ("id" = i64, Path, description = "Class Level ID")
    ),
    responses(
        (status = 204, description = "Class Leveldeleted"),
        (status = 404, description = "Class Level not found")
    ),
    tag = "Class Level"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state
        .class_level_service
        .delete(id.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
