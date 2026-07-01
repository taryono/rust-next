// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateSettingRequest, SettingResponse, UpdateSettingRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create setting
#[utoipa::path(
    post,
    path = "/api/settings",
    request_body = CreateSettingRequest,
    responses(
        (status = 201, description = "Setting created successfully", body = SettingResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Setting "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateSettingRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .setting_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get setting by ID
#[utoipa::path(
    get,
    path = "/api/settings/{id}",
    params(
        ("id" = i64, Path, description = "Setting ID")
    ),
    responses(
        (status = 200, description = "Setting found", body = SettingResponse),
        (status = 404, description = "Setting not found")
    ),
    tag = "Setting "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.setting_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all settings with pagination
#[utoipa::path(
    get,
    path = "/api/settings",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of settings", body = PaginatedResponse<SettingResponse>)
    ),
    tag = "Setting "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.setting_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update setting
#[utoipa::path(
    put,
    path = "/api/settings/{id}",
    params(
        ("id" = i64, Path, description = "Setting ID")
    ),
    request_body = UpdateSettingRequest,
    responses(
        (status = 200, description = "Setting updated", body = SettingResponse),
        (status = 404, description = "Setting not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Setting "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateSettingRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .setting_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete setting
#[utoipa::path(
    delete,
    path = "/api/settings/{id}",
    params(
        ("id" = i64, Path, description = "Setting ID")
    ),
    responses(
        (status = 204, description = "Setting deleted"),
        (status = 404, description = "Setting not found")
    ),
    tag = "Setting "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.setting_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
