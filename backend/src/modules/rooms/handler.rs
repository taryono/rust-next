// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateRoomRequest, RoomResponse, UpdateRoomRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use actix_web::{web, HttpResponse};

/// Create room
#[utoipa::path(
    post,
    path = "/api/rooms",
    request_body = CreateRoomRequest,
    responses(
        (status = 201, description = "Room created successfully", body = RoomResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Room "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateRoomRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.room_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get room by ID
#[utoipa::path(
    get,
    path = "/api/rooms/{id}",
    params(
        ("id" = i64, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "Room found", body = RoomResponse),
        (status = 404, description = "Room not found")
    ),
    tag = "Room "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state.room_service.get_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all rooms with pagination
#[utoipa::path(
    get,
    path = "/api/rooms",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of rooms", body = PaginatedResponse<RoomResponse>)
    ),
    tag = "Room "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    // Optional: foundation_id dari auth/context
    // foundation_id: web::ReqData<i64>,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    // Untuk admin (semua foundation)
    let result = app_state.room_service.get_all(params, None).await?;

    Ok(HttpResponse::Ok().json(result))
}

/// Update room
#[utoipa::path(
    put,
    path = "/api/rooms/{id}",
    params(
        ("id" = i64, Path, description = "Room ID")
    ),
    request_body = UpdateRoomRequest,
    responses(
        (status = 200, description = "Room updated", body = RoomResponse),
        (status = 404, description = "Room not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Room "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateRoomRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .room_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete room
#[utoipa::path(
    delete,
    path = "/api/rooms/{id}",
    params(
        ("id" = i64, Path, description = "Room ID")
    ),
    responses(
        (status = 204, description = "Room deleted"),
        (status = 404, description = "Room not found")
    ),
    tag = "Room "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.room_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
