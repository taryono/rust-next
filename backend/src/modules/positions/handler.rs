// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreatePositionRequest, PositionResponse, UpdatePositionRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};

/// Create position
#[utoipa::path(
    post,
    path = "/api/positions",
    request_body = CreatePositionRequest,
    responses(
        (status = 201, description = "Position created successfully", body = PositionResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Position "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreatePositionRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .position_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get position by ID
#[utoipa::path(
    get,
    path = "/api/positions/{id}",
    params(
        ("id" = i64, Path, description = "Position ID")
    ),
    responses(
        (status = 200, description = "Position found", body = PositionResponse),
        (status = 404, description = "Position not found")
    ),
    tag = "Position "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .position_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all positions with pagination
#[utoipa::path(
    get,
    path = "/api/positions",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of positions", body = PaginatedResponse<PositionResponse>)
    ),
    tag = "Position "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    match app_state.position_service.get_all(params, None).await {
        Ok(positions) => Ok(HttpResponse::Ok().json(ApiResponse::success(positions))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())))
        }
    }
}

/// Update position
#[utoipa::path(
    put,
    path = "/api/positions/{id}",
    params(
        ("id" = i64, Path, description = "Position ID")
    ),
    request_body = UpdatePositionRequest,
    responses(
        (status = 200, description = "Position updated", body = PositionResponse),
        (status = 404, description = "Position not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Position "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdatePositionRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .position_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete position
#[utoipa::path(
    delete,
    path = "/api/positions/{id}",
    params(
        ("id" = i64, Path, description = "Position ID")
    ),
    responses(
        (status = 204, description = "Position deleted"),
        (status = 404, description = "Position not found")
    ),
    tag = "Position "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.position_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
