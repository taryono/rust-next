// ============================================================================
// api/src/modules/menus/handler.rs
// ============================================================================
use super::dto::{CreateMenuRequest, MenuResponse, MenuTree, UpdateMenuRequest};
use crate::app_state::AppState;
use crate::context::ServiceContext;
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};

#[utoipa::path(
    post,
    path = "/api/menus",
    tag = "Menus",
    request_body = CreateMenuRequest,
    responses(
        (status = 201, description = "Menu created successfully", body = MenuResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Menu with this label already exists"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn create(
    app_state: web::Data<AppState>,
    ctx: ServiceContext,
    request: web::Json<CreateMenuRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .menu_service
        .create(&ctx, request.into_inner())
        .await?;

    // ✅ konsisten pakai ApiResponse::success
    Ok(HttpResponse::Created().json(ApiResponse::success(result)))
}

#[utoipa::path(
    get,
    path = "/api/menus/{id}",
    tag = "Menus",
    params(("id" = i64, Path, description = "Menu ID")),
    responses(
        (status = 200, description = "Menu found", body = MenuResponse),
        (status = 404, description = "Menu not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.menu_service.get_by_id(id.into_inner()).await?;

    // ✅ konsisten pakai ApiResponse::success
    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

#[utoipa::path(
    get,
    path = "/api/menus",
    tag = "Menus",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of menus", body = PaginatedResponse<MenuResponse>),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    ctx: ServiceContext,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    // ✅ pakai ? operator, konsisten dengan handler lain
    let menus = app_state
        .menu_service
        .get_all(&ctx, query.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(menus)))
}

#[utoipa::path(
    put,
    path = "/api/menus/{id}",
    tag = "Menus",
    params(("id" = i64, Path, description = "Menu ID")),
    request_body = UpdateMenuRequest,
    responses(
        (status = 200, description = "Menu updated successfully", body = MenuResponse),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Menu not found"),
        (status = 409, description = "Menu with this label already exists"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateMenuRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .menu_service
        .update(id.into_inner(), request.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(result)))
}

#[utoipa::path(
    delete,
    path = "/api/menus/{id}",
    tag = "Menus",
    params(("id" = i64, Path, description = "Menu ID")),
    responses(
        (status = 204, description = "Menu deleted successfully"),
        (status = 404, description = "Menu not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.menu_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/api/menus/my-menus",
    tag = "Menus",
    responses(
        (status = 200, description = "Menus retrieved successfully", body = Vec<MenuTree>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_my_menus(
    app_state: web::Data<AppState>,
    ctx: ServiceContext,
) -> Result<HttpResponse, AppError> {
    let menus = app_state
        .menu_service
        .get_menus_for_user(ctx.user_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(menus)))
}
