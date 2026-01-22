// backend/src/modules/user_profiles/handler.rs
// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateUserProfileRequest, UpdateUserProfileRequest, UserProfileResponse};
use super::service::UserProfileService;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create user_profile
#[utoipa::path(
    post,
    path = "/api/user_profiles",
    request_body = CreateUserProfileRequest,
    responses(
        (status = 201, description = "UserProfile created successfully", body = UserProfileResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "UserProfile "
)]
pub async fn create(
    service: web::Data<UserProfileService>,
    request: web::Json<CreateUserProfileRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get user_profile by ID
#[utoipa::path(
    get,
    path = "/api/user_profiles/{id}",
    params(
        ("id" = i64, Path, description = "UserProfile ID")
    ),
    responses(
        (status = 200, description = "UserProfile found", body = UserProfileResponse),
        (status = 404, description = "UserProfile not found")
    ),
    tag = "UserProfile "
)]
pub async fn get_by_id(
    service: web::Data<UserProfileService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all user_profiles with pagination
#[utoipa::path(
    get,
    path = "/api/user_profiles",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of user_profiles", body = PaginatedResponse<UserProfileResponse>)
    ),
    tag = "UserProfile "
)]
pub async fn get_all(
    service: web::Data<UserProfileService>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update user_profile
#[utoipa::path(
    put,
    path = "/api/user_profiles/{id}",
    params(
        ("id" = i64, Path, description = "UserProfile ID")
    ),
    request_body = UpdateUserProfileRequest,
    responses(
        (status = 200, description = "UserProfile updated", body = UserProfileResponse),
        (status = 404, description = "UserProfile not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "UserProfile "
)]
pub async fn update(
    service: web::Data<UserProfileService>,
    id: web::Path<i64>,
    request: web::Json<UpdateUserProfileRequest>,
) -> Result<HttpResponse, AppError> {
    let result = service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete user_profile
#[utoipa::path(
    delete,
    path = "/api/user_profiles/{id}",
    params(
        ("id" = i64, Path, description = "UserProfile ID")
    ),
    responses(
        (status = 204, description = "UserProfile deleted"),
        (status = 404, description = "UserProfile not found")
    ),
    tag = "UserProfile "
)]
pub async fn delete(
    service: web::Data<UserProfileService>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
