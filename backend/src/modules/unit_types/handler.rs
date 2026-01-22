// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateUnitTypeRequest, UnitTypeResponse, UpdateUnitTypeRequest};
use super::service::UnitTypeService;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create unit_type
#[utoipa::path(
    post,
    path = "/api/unit_types",
    request_body = CreateUnitTypeRequest,
    responses(
        (status = 201, description = "UnitType created successfully", body = UnitTypeResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "UnitType "
)]
pub async fn create(
    service: web::Data<UnitTypeService>,
    request: web::Json<CreateUnitTypeRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get unit_type by ID
#[utoipa::path(
    get,
    path = "/api/unit_types/{id}",
    params(
        ("id" = i64, Path, description = "UnitType ID")
    ),
    responses(
        (status = 200, description = "UnitType found", body = UnitTypeResponse),
        (status = 404, description = "UnitType not found")
    ),
    tag = "UnitType "
)]
pub async fn get_by_id(
    service: web::Data<UnitTypeService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all unit_types with pagination
#[utoipa::path(
    get,
    path = "/api/unit_types",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of unit_types", body = PaginatedResponse<UnitTypeResponse>)
    ),
    tag = "UnitType "
)]
pub async fn get_all(
    service: web::Data<UnitTypeService>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update unit_type
#[utoipa::path(
    put,
    path = "/api/unit_types/{id}",
    params(
        ("id" = i64, Path, description = "UnitType ID")
    ),
    request_body = UpdateUnitTypeRequest,
    responses(
        (status = 200, description = "UnitType updated", body = UnitTypeResponse),
        (status = 404, description = "UnitType not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "UnitType "
)]
pub async fn update(
    service: web::Data<UnitTypeService>,
    id: web::Path<i64>,
    request: web::Json<UpdateUnitTypeRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete unit_type
#[utoipa::path(
    delete,
    path = "/api/unit_types/{id}",
    params(
        ("id" = i64, Path, description = "UnitType ID")
    ),
    responses(
        (status = 204, description = "UnitType deleted"),
        (status = 404, description = "UnitType not found")
    ),
    tag = "UnitType "
)]
pub async fn delete(
    service: web::Data<UnitTypeService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
