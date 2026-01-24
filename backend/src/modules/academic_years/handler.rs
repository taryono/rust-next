// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{AcademicYearResponse, CreateAcademicYearRequest, UpdateAcademicYearRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};
/// Create academic year
#[utoipa::path(
    post,
    path = "/api/academic-years",
    request_body = CreateAcademicYearRequest,
    responses(
        (status = 201, description = "Academic year created successfully", body = AcademicYearResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Academic Years"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateAcademicYearRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .academic_year_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get academic year by ID
#[utoipa::path(
    get,
    path = "/api/academic-years/{id}",
    params(
        ("id" = i64, Path, description = "Academic year ID")
    ),
    responses(
        (status = 200, description = "Academic year found", body = AcademicYearResponse),
        (status = 404, description = "Academic year not found")
    ),
    tag = "Academic Years"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .academic_year_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all academic years with pagination
#[utoipa::path(
    get,
    path = "/api/academic-years",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of academic years", body = PaginatedResponse<AcademicYearResponse>)
    ),
    tag = "Academic Years"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();

    // Jika pakai multi-tenant by foundation
    // let result = app_state.academic_year_app_state.permission_serviceget_all(params, Some(*foundation_id)).await?;

    // Untuk admin (semua foundation)
    let result = app_state
        .academic_year_service
        .get_all(params, None)
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Get active academic year
#[utoipa::path(
    get,
    path = "/api/academic-years/active/{foundation_id}",
    params(
        ("foundation_id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 200, description = "Active academic year", body = AcademicYearResponse),
        (status = 404, description = "No active academic year found")
    ),
    tag = "Academic Years"
)]
pub async fn get_active(
    app_state: web::Data<AppState>,
    foundation_id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .academic_year_service
        .get_active(foundation_id.into_inner())
        .await?;
    match result {
        Some(data) => Ok(HttpResponse::Ok().json(data)),
        None => Err(AppError::NotFoundError(
            "No active academic year found".to_string(),
        )),
    }
}

/// Update academic year
#[utoipa::path(
    put,
    path = "/api/academic-years/{id}",
    params(
        ("id" = i64, Path, description = "Academic year ID")
    ),
    request_body = UpdateAcademicYearRequest,
    responses(
        (status = 200, description = "Academic year updated", body = AcademicYearResponse),
        (status = 404, description = "Academic year not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Academic Years"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateAcademicYearRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .academic_year_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete academic year
#[utoipa::path(
    delete,
    path = "/api/academic-years/{id}",
    params(
        ("id" = i64, Path, description = "Academic year ID")
    ),
    responses(
        (status = 204, description = "Academic year deleted"),
        (status = 404, description = "Academic year not found")
    ),
    tag = "Academic Years"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state
        .academic_year_service
        .delete(id.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
