// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{ApplicantResponse, CreateApplicantRequest, UpdateApplicantRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create applicant
#[utoipa::path(
    post,
    path = "/api/applicants",
    request_body = CreateApplicantRequest,
    responses(
        (status = 201, description = "Applicant created successfully", body = ApplicantResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Applicant "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateApplicantRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .applicant_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get applicant by ID
#[utoipa::path(
    get,
    path = "/api/applicants/{id}",
    params(
        ("id" = i64, Path, description = "Applicant ID")
    ),
    responses(
        (status = 200, description = "Applicant found", body = ApplicantResponse),
        (status = 404, description = "Applicant not found")
    ),
    tag = "Applicant "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .applicant_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all applicants with pagination
#[utoipa::path(
    get,
    path = "/api/applicants",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of applicants", body = PaginatedResponse<ApplicantResponse>)
    ),
    tag = "Applicant "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.applicant_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update applicant
#[utoipa::path(
    put,
    path = "/api/applicants/{id}",
    params(
        ("id" = i64, Path, description = "Applicant ID")
    ),
    request_body = UpdateApplicantRequest,
    responses(
        (status = 200, description = "Applicant updated", body = ApplicantResponse),
        (status = 404, description = "Applicant not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Applicant "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateApplicantRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .applicant_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete applicant
#[utoipa::path(
    delete,
    path = "/api/applicants/{id}",
    params(
        ("id" = i64, Path, description = "Applicant ID")
    ),
    responses(
        (status = 204, description = "Applicant deleted"),
        (status = 404, description = "Applicant not found")
    ),
    tag = "Applicant "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.applicant_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
