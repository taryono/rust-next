// ============================================================================
// src/modules/guardians/handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateGuardianRequest, GuardianResponse, UpdateGuardianRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create guardian
#[utoipa::path(
    post,
    path = "/api/guardians",
    request_body = CreateGuardianRequest,
    responses(
        (status = 201, description = "Guardian created successfully", body = GuardianResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Guardian "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateGuardianRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .guardian_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get guardian by ID
#[utoipa::path(
    get,
    path = "/api/guardians/{id}",
    params(
        ("id" = i64, Path, description = "Guardian ID")
    ),
    responses(
        (status = 200, description = "Guardian found", body = GuardianResponse),
        (status = 404, description = "Guardian not found")
    ),
    tag = "Guardian "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .guardian_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all guardians with pagination
#[utoipa::path(
    get,
    path = "/api/guardians",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of guardians", body = PaginatedResponse<GuardianResponse>)
    ),
    tag = "Guardian "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.guardian_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update guardian
#[utoipa::path(
    put,
    path = "/api/guardians/{id}",
    params(
        ("id" = i64, Path, description = "Guardian ID")
    ),
    request_body = UpdateGuardianRequest,
    responses(
        (status = 200, description = "Guardian updated", body = GuardianResponse),
        (status = 404, description = "Guardian not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Guardian "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateGuardianRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .guardian_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete guardian
#[utoipa::path(
    delete,
    path = "/api/guardians/{id}",
    params(
        ("id" = i64, Path, description = "Guardian ID")
    ),
    responses(
        (status = 204, description = "Guardian deleted"),
        (status = 404, description = "Guardian not found")
    ),
    tag = "Guardian "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.guardian_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
