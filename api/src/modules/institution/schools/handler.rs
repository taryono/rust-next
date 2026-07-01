// api/src/modules/institution/school/handler.rs

// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateSchoolRequest, SchoolResponse, UpdateSchoolRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create School
#[utoipa::path(
    post,
    path = "/api/schools",
    request_body = CreateSchoolRequest,
    responses(
        (status = 201, description = "School created successfully", body = SchoolResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "School "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateSchoolRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .school_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get School by ID
#[utoipa::path(
    get,
    path = "/api/schools/{id}",
    params(
        ("id" = i64, Path, description = "School ID")
    ),
    responses(
        (status = 200, description = "School found", body = SchoolResponse),
        (status = 404, description = "School not found")
    ),
    tag = "School "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.school_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all Schools with pagination
#[utoipa::path(
    get,
    path = "/api/schools",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of Schools", body = PaginatedResponse<SchoolResponse>)
    ),
    tag = "School "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.school_service.get_all(params).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update School
#[utoipa::path(
    put,
    path = "/api/schools/{id}",
    params(
        ("id" = i64, Path, description = "School ID")
    ),
    request_body = UpdateSchoolRequest,
    responses(
        (status = 200, description = "School updated", body = SchoolResponse),
        (status = 404, description = "School not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "School "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateSchoolRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .school_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete School
#[utoipa::path(
    delete,
    path = "/api/schools/{id}",
    params(
        ("id" = i64, Path, description = "School ID")
    ),
    responses(
        (status = 204, description = "School deleted"),
        (status = 404, description = "School not found")
    ),
    tag = "School "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.school_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
