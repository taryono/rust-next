// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateMenuRequest, MenuResponse, UpdateMenuRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};

/// Create menu
#[utoipa::path(
    post,
    path = "/api/menus",
    request_body = CreateMenuRequest,
    responses(
        (status = 201, description = "Menu created successfully", body = MenuResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Menu "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateMenuRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.menu_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get menu by ID
#[utoipa::path(
    get,
    path = "/api/menus/{id}",
    params(
        ("id" = i64, Path, description = "Menu ID")
    ),
    responses(
        (status = 200, description = "Menu found", body = MenuResponse),
        (status = 404, description = "Menu not found")
    ),
    tag = "Menu "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.menu_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all menus with pagination
#[utoipa::path(
    get,
    path = "/api/menus",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of menus", body = PaginatedResponse<MenuResponse>)
    ),
    tag = "Menu "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    match app_state.menu_service.get_all(params, None).await {
        Ok(menus) => Ok(HttpResponse::Ok().json(ApiResponse::success(menus))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())))
        }
    }
}

/// Update menu
#[utoipa::path(
    put,
    path = "/api/menus/{id}",
    params(
        ("id" = i64, Path, description = "Menu ID")
    ),
    request_body = UpdateMenuRequest,
    responses(
        (status = 200, description = "Menu updated", body = MenuResponse),
        (status = 404, description = "Menu not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Menu "
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
    Ok(HttpResponse::Ok().json(result))
}

/// Delete menu
#[utoipa::path(
    delete,
    path = "/api/menus/{id}",
    params(
        ("id" = i64, Path, description = "Menu ID")
    ),
    responses(
        (status = 204, description = "Menu deleted"),
        (status = 404, description = "Menu not found")
    ),
    tag = "Menu "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.menu_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_my_menus(
    app_state: web::Data<AppState>,
    user: AuthContext, // middleware auth kamu
) -> Result<Json<Vec<MenuTree>>, AppError> {
    let menus = app_state.menu_service.get_menus_for_user(&user).await?;
    Ok(Json(menus))
}
