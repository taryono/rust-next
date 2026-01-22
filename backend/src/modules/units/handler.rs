// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateUnitRequest, UnitResponse, UpdateUnitRequest};
use super::service::UnitService;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create unit
#[utoipa::path(
    post,
    path = "/api/units",
    request_body = CreateUnitRequest,
    responses(
        (status = 201, description = "Unit created successfully", body = UnitResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Unit "
)]
pub async fn create(
    service: web::Data<UnitService>,
    request: web::Json<CreateUnitRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get unit by ID
#[utoipa::path(
    get,
    path = "/api/units/{id}",
    params(
        ("id" = i64, Path, description = "Unit ID")
    ),
    responses(
        (status = 200, description = "Unit found", body = UnitResponse),
        (status = 404, description = "Unit not found")
    ),
    tag = "Unit "
)]
pub async fn get_by_id(
    service: web::Data<UnitService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all units with pagination
#[utoipa::path(
    get,
    path = "/api/units",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of units", body = PaginatedResponse<UnitResponse>)
    ),
    tag = "Unit "
)]
pub async fn get_all(
    service: web::Data<UnitService>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update unit
#[utoipa::path(
    put,
    path = "/api/units/{id}",
    params(
        ("id" = i64, Path, description = "Unit ID")
    ),
    request_body = UpdateUnitRequest,
    responses(
        (status = 200, description = "Unit updated", body = UnitResponse),
        (status = 404, description = "Unit not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Unit "
)]
pub async fn update(
    service: web::Data<UnitService>,
    id: web::Path<i64>,
    request: web::Json<UpdateUnitRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete unit
#[utoipa::path(
    delete,
    path = "/api/units/{id}",
    params(
        ("id" = i64, Path, description = "Unit ID")
    ),
    responses(
        (status = 204, description = "Unit deleted"),
        (status = 404, description = "Unit not found")
    ),
    tag = "Unit "
)]
pub async fn delete(
    service: web::Data<UnitService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
