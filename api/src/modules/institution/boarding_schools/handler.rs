// api/src/modules/institution/boarding_schools/handler.rs
// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{
    BoardingSchoolResponse, CreateBoardingSchoolRequest, UpdateBoardingSchoolRequest,
};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create BoardingSchool
#[utoipa::path(
    post,
    path = "/api/boarding_schools",
    request_body = CreateBoardingSchoolRequest,
    responses(
        (status = 201, description = "BoardingSchool created successfully", body = BoardingSchoolResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "BoardingSchool "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateBoardingSchoolRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .boarding_school_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get BoardingSchool by ID
#[utoipa::path(
    get,
    path = "/api/boarding_schools/{id}",
    params(
        ("id" = i64, Path, description = "BoardingSchool ID")
    ),
    responses(
        (status = 200, description = "BoardingSchool found", body = BoardingSchoolResponse),
        (status = 404, description = "BoardingSchool not found")
    ),
    tag = "BoardingSchool "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .boarding_school_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all BoardingSchools with pagination
#[utoipa::path(
    get,
    path = "/api/boarding_schools",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of Boarding Schooles", body = PaginatedResponse<BoardingSchoolResponse>)
    ),
    tag = "BoardingSchool "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.boarding_school_service.get_all(params).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update BoardingSchool
#[utoipa::path(
    put,
    path = "/api/boarding_schools/{id}",
    params(
        ("id" = i64, Path, description = "BoardingSchool ID")
    ),
    request_body = UpdateBoardingSchoolRequest,
    responses(
        (status = 200, description = "BoardingSchool updated", body = BoardingSchoolResponse),
        (status = 404, description = "BoardingSchool not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "BoardingSchool "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateBoardingSchoolRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .boarding_school_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete BoardingSchool
#[utoipa::path(
    delete,
    path = "/api/boarding_schools/{id}",
    params(
        ("id" = i64, Path, description = "BoardingSchool ID")
    ),
    responses(
        (status = 204, description = "BoardingSchool deleted"),
        (status = 404, description = "BoardingSchool not found")
    ),
    tag = "BoardingSchool "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state
        .boarding_school_service
        .delete(id.into_inner())
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
