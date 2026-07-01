// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateRegistrationRequest, RegistrationResponse, UpdateRegistrationRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create registration
#[utoipa::path(
    post,
    path = "/api/registrations",
    request_body = CreateRegistrationRequest,
    responses(
        (status = 201, description = "Registration created successfully", body = RegistrationResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Registration "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateRegistrationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .registration_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get registration by ID
#[utoipa::path(
    get,
    path = "/api/registrations/{id}",
    params(
        ("id" = i64, Path, description = "Registration ID")
    ),
    responses(
        (status = 200, description = "Registration found", body = RegistrationResponse),
        (status = 404, description = "Registration not found")
    ),
    tag = "Registration "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .registration_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all registrations with pagination
#[utoipa::path(
    get,
    path = "/api/registrations",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of registrations", body = PaginatedResponse<RegistrationResponse>)
    ),
    tag = "Registration "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.registration_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update registration
#[utoipa::path(
    put,
    path = "/api/registrations/{id}",
    params(
        ("id" = i64, Path, description = "Registration ID")
    ),
    request_body = UpdateRegistrationRequest,
    responses(
        (status = 200, description = "Registration updated", body = RegistrationResponse),
        (status = 404, description = "Registration not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Registration "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateRegistrationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .registration_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete registration
#[utoipa::path(
    delete,
    path = "/api/registrations/{id}",
    params(
        ("id" = i64, Path, description = "Registration ID")
    ),
    responses(
        (status = 204, description = "Registration deleted"),
        (status = 404, description = "Registration not found")
    ),
    tag = "Registration "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state
        .registration_service
        .delete(id.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
