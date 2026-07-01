// ============================================================================
// handler.rs - HTTP Handlers
// api/src/modules/foundation_regulations/handler.rs
// ============================================================================
use super::dto::{
    CreateFoundationRegulationRequest, FoundationRegulationResponse,
    ToggleFoundationRegulationRequest, UpdateFoundationRegulationConfigRequest,
};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};

// ============================================================================
// GET SEMUA REGULASI PER YAYASAN
// ============================================================================

/// Get semua regulasi beserta status aktif untuk yayasan tertentu
#[utoipa::path(
    get,
    path = "/api/foundations/{foundation_code}/regulations",
    params(
        ("foundation_code" = String, Path, description = "Foundation code e.g. YAY-A")
    ),
    responses(
        (status = 200, description = "List regulations for foundation", body = Vec<FoundationRegulationResponse>),
        (status = 404, description = "Foundation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn get_by_foundation_code(
    app_state: web::Data<AppState>,
    foundation_code: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_regulation_service
        .get_by_foundation_code(foundation_code.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

// ============================================================================
// ADMIN — GET ALL (dengan pagination)
// ============================================================================

/// Get all foundation regulations (admin)
#[utoipa::path(
    get,
    path = "/api/foundation-regulations",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10)"),
        ("foundation_id" = Option<i64>, Query, description = "Filter by foundation ID"),
    ),
    responses(
        (status = 200, description = "List of foundation regulations", body = PaginatedResponse<FoundationRegulationResponse>)
    ),
    tag = "Foundation Regulations"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    foundation_id: web::Query<Option<i64>>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_regulation_service
        .get_all(query.into_inner(), foundation_id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

/// Get foundation regulation by ID
#[utoipa::path(
    get,
    path = "/api/foundation-regulations/{id}",
    params(
        ("id" = i64, Path, description = "Foundation Regulation ID")
    ),
    responses(
        (status = 200, description = "Foundation regulation found", body = FoundationRegulationResponse),
        (status = 404, description = "Not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .foundation_regulation_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

// ============================================================================
// CREATE — Assign regulasi ke yayasan
// ============================================================================

/// Assign regulasi ke yayasan
#[utoipa::path(
    post,
    path = "/api/foundations/{foundation_code}/regulations/{regulation_code}",
    params(
        ("foundation_code" = String, Path, description = "Foundation code e.g. YAY-A"),
        ("regulation_code" = String, Path, description = "Regulation code e.g. PAYMENT_INSTALLMENT")
    ),
    request_body = CreateFoundationRegulationRequest,
    responses(
        (status = 201, description = "Regulation assigned to foundation", body = FoundationRegulationResponse),
        (status = 404, description = "Foundation or regulation not found"),
        (status = 409, description = "Already assigned")
    ),
    tag = "Foundation Regulations"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    request: web::Json<CreateFoundationRegulationRequest>,
) -> Result<HttpResponse, AppError> {
    let (foundation_code, regulation_code) = path.into_inner();
    let result = app_state
        .foundation_regulation_service
        .create(foundation_code, regulation_code, request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(result)))
}

// ============================================================================
// TOGGLE — Aktifkan / Nonaktifkan regulasi
// ============================================================================

/// Toggle aktif/nonaktif regulasi untuk yayasan tertentu
#[utoipa::path(
    put,
    path = "/api/foundations/{foundation_code}/regulations/{regulation_code}/toggle",
    params(
        ("foundation_code" = String, Path, description = "Foundation code"),
        ("regulation_code" = String, Path, description = "Regulation code")
    ),
    request_body = ToggleFoundationRegulationRequest,
    responses(
        (status = 200, description = "Regulation toggled", body = FoundationRegulationResponse),
        (status = 404, description = "Foundation or regulation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn toggle(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    request: web::Json<ToggleFoundationRegulationRequest>,
) -> Result<HttpResponse, AppError> {
    let (foundation_code, regulation_code) = path.into_inner();
    let result = app_state
        .foundation_regulation_service
        .toggle(foundation_code, regulation_code, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

// ============================================================================
// UPDATE CONFIG
// ============================================================================

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
        (status = 400, description = "Regulation is inactive"),
        (status = 404, description = "Foundation or regulation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn update_config(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    request: web::Json<UpdateFoundationRegulationConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let (foundation_code, regulation_code) = path.into_inner();
    let result = app_state
        .foundation_regulation_service
        .update_config(foundation_code, regulation_code, request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

// ============================================================================
// SOFT DELETE
// ============================================================================

/// Hapus (soft delete) regulasi dari yayasan
#[utoipa::path(
    delete,
    path = "/api/foundations/{foundation_code}/regulations/{regulation_code}",
    params(
        ("foundation_code" = String, Path, description = "Foundation code"),
        ("regulation_code" = String, Path, description = "Regulation code")
    ),
    responses(
        (status = 204, description = "Regulation removed from foundation"),
        (status = 404, description = "Foundation or regulation not found")
    ),
    tag = "Foundation Regulations"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (foundation_code, regulation_code) = path.into_inner();
    app_state
        .foundation_regulation_service
        .delete(foundation_code, regulation_code)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
