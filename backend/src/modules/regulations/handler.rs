// backend/src/modules/regulations/handler.rs
use super::dto::{
    CreateRegulationRequest, FoundationRegulationResponse, RegulationResponse,
    ToggleRegulationRequest, UpdateFoundationRegulationConfigRequest, UpdateRegulationRequest,
};
use crate::app_state::AppState; 
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};

// ============================================================================
// MASTER REGULATION HANDLERS (Admin)
// ============================================================================

/// Create regulation
#[utoipa::path(
    post,
    path = "/api/regulations",
    request_body = CreateRegulationRequest,
    responses(
        (status = 201, description = "Regulation created", body = RegulationResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate code")
    ),
    tag = "Regulations"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateRegulationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .regulation_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(result)))
}

/// Get regulation by ID
#[utoipa::path(
    get,
    path = "/api/regulations/{id}",
    params(("id" = i64, Path, description = "Regulation ID")),
    responses(
        (status = 200, description = "Regulation found", body = RegulationResponse),
        (status = 404, description = "Regulation not found")
    ),
    tag = "Regulations"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .regulation_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// Get all regulations with pagination
#[utoipa::path(
    get,
    path = "/api/regulations",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10)"),
        ("search" = Option<String>, Query, description = "Search by name or code"),
    ),
    responses(
        (status = 200, description = "List of regulations", body = PaginatedResponse<RegulationResponse>)
    ),
    tag = "Regulations"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .regulation_service
        .get_all(query.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// Update regulation
#[utoipa::path(
    put,
    path = "/api/regulations/{id}",
    params(("id" = i64, Path, description = "Regulation ID")),
    request_body = UpdateRegulationRequest,
    responses(
        (status = 200, description = "Regulation updated", body = RegulationResponse),
        (status = 404, description = "Regulation not found")
    ),
    tag = "Regulations"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateRegulationRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .regulation_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// Delete regulation
#[utoipa::path(
    delete,
    path = "/api/regulations/{id}",
    params(("id" = i64, Path, description = "Regulation ID")),
    responses(
        (status = 204, description = "Regulation deleted"),
        (status = 404, description = "Regulation not found")
    ),
    tag = "Regulations"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.regulation_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

// ============================================================================
// FOUNDATION REGULATION HANDLERS (Per Yayasan)
// ============================================================================

/// Get semua regulasi beserta status aktif untuk yayasan tertentu
#[utoipa::path(
    get,
    path = "/api/foundations/{foundation_code}/regulations",
    params(("foundation_code" = String, Path, description = "Foundation code")),
    responses(
        (status = 200, description = "List regulations for foundation", body = Vec<FoundationRegulationResponse>),
        (status = 404, description = "Foundation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn get_foundation_regulations(
    app_state: web::Data<AppState>,
    foundation_code: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .regulation_service
        .get_by_foundation_code(foundation_code.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// Toggle aktif/nonaktif regulasi untuk yayasan tertentu
#[utoipa::path(
    put,
    path = "/api/foundations/{foundation_code}/regulations/{regulation_code}/toggle",
    params(
        ("foundation_code" = String, Path, description = "Foundation code"),
        ("regulation_code" = String, Path, description = "Regulation code e.g. PAYMENT_INSTALLMENT")
    ),
    request_body = ToggleRegulationRequest,
    responses(
        (status = 200, description = "Regulation toggled", body = FoundationRegulationResponse),
        (status = 404, description = "Foundation or regulation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn toggle_foundation_regulation(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    request: web::Json<ToggleRegulationRequest>,
) -> Result<HttpResponse, AppError> {
    let (foundation_code, regulation_code) = path.into_inner();
    let result = app_state
        .regulation_service
        .toggle(foundation_code, regulation_code, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// Update config regulasi untuk yayasan tertentu
#[utoipa::path(
    put,
    path = "/api/foundations/{foundation_code}/regulations/{regulation_code}/config",
    params(
        ("foundation_code" = String, Path, description = "Foundation code"),
        ("regulation_code" = String, Path, description = "Regulation code")
    ),
    request_body = UpdateFoundationRegulationConfigRequest,
    responses(
        (status = 200, description = "Config updated", body = FoundationRegulationResponse),
        (status = 404, description = "Foundation or regulation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn update_foundation_regulation_config(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    request: web::Json<UpdateFoundationRegulationConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let (foundation_code, regulation_code) = path.into_inner();
    let result = app_state
        .regulation_service
        .update_config(foundation_code, regulation_code, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

// ============================================================================
// FOUNDATION REGULATION HANDLERS (Per Yayasan)
// ============================================================================
