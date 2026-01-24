// src/modules/auth/handler.rs
use crate::app_state::AppState;
use crate::modules::auth::dto::{
    AuthResponse, LoginRequest, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
};
use crate::utils::response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;

/// Register new user handler
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists")
    )
)]
pub async fn register(
    app_state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> impl Responder {
    // Validate input
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": format!("Validation error: {}", e)
        }));
    }

    match app_state.auth_service.register(payload.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Login user handler
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    app_state: web::Data<AppState>,
    payload: web::Json<LoginRequest>,
) -> impl Responder {
    // Validate input
    if let Err(e) = payload.validate() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": format!("Validation error: {}", e)
        }));
    }

    match app_state.auth_service.login(payload.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Refresh token handler
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
pub async fn refresh(
    app_state: web::Data<AppState>,
    payload: web::Json<RefreshTokenRequest>,
) -> impl Responder {
    match app_state
        .auth_service
        .refresh_token(payload.refresh_token.clone())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string())),
    }
}
