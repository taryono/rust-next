// ============================================
// backend/src/controllers/user_controller.rs
// ============================================
use crate::utils::pagination::PaginationParams;
use crate::{
    app_state::AppState,
    modules::users::dto::{
        ChangePasswordRequest, UpdateUserRequest, UserListResponse, UserResponse,
    },
    utils::{jwt::Claims, response::ApiResponse},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(
        ("page" = Option<i64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<i64>, Query, description = "Items per page, default 10, max 100"),
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
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse> {
    match app_state
        .user_service
        .get_all(query.into_inner(), None)
        .await
    {
        Ok(users) => Ok(HttpResponse::Ok().json(ApiResponse::success(users))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = i64, Path, description = "User ID")
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
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match app_state.user_service.get_by_id(user_id).await {
        Ok(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user))),
        Err(e) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string()))),
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
pub async fn get_current_user(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let user_id: i64 = claims.sub.parse().unwrap_or(0);

            match app_state.user_service.get_by_id(user_id).await {
                Ok(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user))),
                Err(e) => {
                    Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string())))
                }
            }
        }
        None => {
            Ok(HttpResponse::Unauthorized()
                .json(ApiResponse::<()>::error("Unauthorized".to_string())))
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
pub async fn update_user(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(errors) = body.validate() {
        return Ok(
            HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!(
                "Validation error: {}",
                errors
            ))),
        );
    }

    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let user_id: i64 = claims.sub.parse().unwrap_or(0);

            match app_state
                .user_service
                .update(user_id, body.into_inner())
                .await
            {
                Ok(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user))),
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())))
                }
            }
        }
        None => {
            Ok(HttpResponse::Unauthorized()
                .json(ApiResponse::<()>::error("Unauthorized".to_string())))
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
    app_state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(errors) = body.validate() {
        return Ok(
            HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!(
                "Validation error: {}",
                errors
            ))),
        );
    }

    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let user_id: i64 = claims.sub.parse().unwrap_or(0);

            match app_state
                .user_service
                .change_password(user_id, body.into_inner())
                .await
            {
                Ok(_) => {
                    Ok(HttpResponse::Ok()
                        .json(ApiResponse::success("Password changed successfully")))
                }
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())))
                }
            }
        }
        None => {
            Ok(HttpResponse::Unauthorized()
                .json(ApiResponse::<()>::error("Unauthorized".to_string())))
        }
    }
}

/// Soft delete user (replaces delete_user)
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User soft deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_user(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match app_state.user_service.soft_delete(user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("User deleted successfully"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Restore deleted user
#[utoipa::path(
    post,
    path = "/api/users/{id}/restore",
    tag = "users",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User restored successfully"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_user(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match app_state.user_service.restore(user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("User restored successfully"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Force delete user (permanent)
#[utoipa::path(
    delete,
    path = "/api/users/{id}/force",
    tag = "users",
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User permanently deleted"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn force_delete_user(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    match app_state.user_service.delete(user_id).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("User permanently deleted"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}
