use crate::{
    config::database::Database,
    models::auth::{AuthResponse, LoginRequest, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest},
    services::auth_service,
    utils::response::ApiResponse,
};
use actix_web::{web, HttpResponse};
use validator::Validate;

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
    db: web::Data<Database>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("Validation error: {}", errors)
        ));
    }

    match auth_service::register_user(db.get_connection(), body.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

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
    db: web::Data<Database>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("Validation error: {}", errors)
        ));
    }

    match auth_service::login_user(db.get_connection(), body.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string())),
    }
}

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
    body: web::Json<RefreshTokenRequest>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("Validation error: {}", errors)
        ));
    }

    match auth_service::refresh_token(body.refresh_token.clone()).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse::success(response)),
        Err(e) => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string())),
    }
}