use crate::{
    config::database::Database,
    models::user::{ChangePasswordRequest, UpdateUserRequest, UserListResponse, UserResponse},
    services::user_service,
    utils::{jwt::Claims, response::ApiResponse},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use validator::Validate;

use crate::models::pagination::PaginationParams;

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(
        ("page" = Option<u64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<u64>, Query, description = "Items per page, default 10, max 100"),
        ("search" = Option<String>, Query, description = "Search by name or email"),
        ("role" = Option<String>, Query, description = "Filter by role")
    ),
    responses(
        (status = 200, description = "List of users retrieved successfully", body = UserListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_users(
    db: web::Data<Database>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    match user_service::get_all_users(db.get_connection(), query.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = u64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_by_id(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    let user_id = path.into_inner();

    match user_service::get_user_by_id(db.get_connection(), user_id).await {
        Ok(user) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let user_id: u64 = claims.sub.parse().unwrap_or(0);

            match user_service::get_user_by_id(db.get_connection(), user_id).await {
                Ok(user) => HttpResponse::Ok().json(ApiResponse::success(user)),
                Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string())),
            }
        }
        None => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Unauthorized".to_string()))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/users/me",
    tag = "users",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_user(
    db: web::Data<Database>,
    req: HttpRequest,
    body: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!(
            "Validation error: {}",
            errors
        )));
    }

    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let user_id: u64 = claims.sub.parse().unwrap_or(0);

            match user_service::update_user(db.get_connection(), user_id, body.into_inner()).await {
                Ok(user) => HttpResponse::Ok().json(ApiResponse::success(user)),
                Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
            }
        }
        None => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Unauthorized".to_string()))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/users/change-password",
    tag = "users",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Validation error or incorrect old password"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn change_password(
    db: web::Data<Database>,
    req: HttpRequest,
    body: web::Json<ChangePasswordRequest>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!(
            "Validation error: {}",
            errors
        )));
    }

    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let user_id: u64 = claims.sub.parse().unwrap_or(0);

            match user_service::change_password(db.get_connection(), user_id, body.into_inner())
                .await
            {
                Ok(_) => {
                    HttpResponse::Ok().json(ApiResponse::success("Password changed successfully"))
                }
                Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
            }
        }
        None => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Unauthorized".to_string()))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = u64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_user(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    let user_id = path.into_inner();

    match user_service::delete_user(db.get_connection(), user_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("User deleted successfully")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}
