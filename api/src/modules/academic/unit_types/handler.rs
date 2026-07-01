// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateUnitTypeRequest, UnitTypeResponse, UpdateUnitTypeRequest};
use crate::app_state::AppState;
use crate::context::ServiceContext;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};
/// Create academic type
#[utoipa::path(
    post,
    path = "/api/unit-types",
    request_body = CreateUnitTypeRequest,
    responses(
        (status = 201, description = "Unit type created successfully", body = UnitTypeResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Unit Types"
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateUnitTypeRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .unit_type_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get academic type by ID
#[utoipa::path(
    get,
    path = "/api/unit-types/{id}",
    params(
        ("id" = i64, Path, description = "Unit type ID")
    ),
    responses(
        (status = 200, description = "Unit type found", body = UnitTypeResponse),
        (status = 404, description = "Unit type not found")
    ),
    tag = "Unit Types"
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .unit_type_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all academic types with pagination
#[utoipa::path(
    get,
    path = "/api/unit-types",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of academic types", body = PaginatedResponse<UnitTypeResponse>)
    ),
    tag = "Unit Types"
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    ctx: ServiceContext,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    let foundation_id = ctx.foundation_id;
    // Jika pakai multi-tenant by foundation
    // let result = app_state.academic_type_app_state.permission_serviceget_all(params, Some(*foundation_id)).await?;

    // Untuk admin (semua foundation)
    let result = app_state
        .unit_type_service
        .get_all(params, foundation_id)
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update academic type
#[utoipa::path(
    put,
    path = "/api/unit-types/{id}",
    params(
        ("id" = i64, Path, description = "Unit type ID")
    ),
    request_body = UpdateUnitTypeRequest,
    responses(
        (status = 200, description = "Unit type updated", body = UnitTypeResponse),
        (status = 404, description = "Unit type not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Unit Types"
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateUnitTypeRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .unit_type_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete academic type
#[utoipa::path(
    delete,
    path = "/api/unit-types/{id}",
    params(
        ("id" = i64, Path, description = "Unit type ID")
    ),
    responses(
        (status = 204, description = "Unit type deleted"),
        (status = 404, description = "Unit type not found")
    ),
    tag = "Unit Types"
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.unit_type_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
