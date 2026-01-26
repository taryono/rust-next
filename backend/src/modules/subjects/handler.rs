// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateSubjectRequest, SubjectResponse, UpdateSubjectRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create subject
#[utoipa::path(
    post,
    path = "/api/subjects",
    request_body = CreateSubjectRequest,
    responses(
        (status = 201, description = "Subject created successfully", body = SubjectResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Subject "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateSubjectRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .subject_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get subject by ID
#[utoipa::path(
    get,
    path = "/api/subjects/{id}",
    params(
        ("id" = i64, Path, description = "Subject ID")
    ),
    responses(
        (status = 200, description = "Subject found", body = SubjectResponse),
        (status = 404, description = "Subject not found")
    ),
    tag = "Subject "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.subject_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all subjects with pagination
#[utoipa::path(
    get,
    path = "/api/subjects",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of subjects", body = PaginatedResponse<SubjectResponse>)
    ),
    tag = "Subject "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.subject_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update subject
#[utoipa::path(
    put,
    path = "/api/subjects/{id}",
    params(
        ("id" = i64, Path, description = "Subject ID")
    ),
    request_body = UpdateSubjectRequest,
    responses(
        (status = 200, description = "Subject updated", body = SubjectResponse),
        (status = 404, description = "Subject not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Subject "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateSubjectRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .subject_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete subject
#[utoipa::path(
    delete,
    path = "/api/subjects/{id}",
    params(
        ("id" = i64, Path, description = "Subject ID")
    ),
    responses(
        (status = 204, description = "Subject deleted"),
        (status = 404, description = "Subject not found")
    ),
    tag = "Subject "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.subject_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
