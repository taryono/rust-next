// ============================================
// backend/src/modules/users/handler.rs
// ============================================
use super::dto_multipart::{CreateUserMultipart, CreateUserMultipartRequest};
use crate::utils::pagination::PaginationParams;
use crate::{
    app_state::AppState,
    errors::AppError,
    modules::users::dto::{
        ChangePasswordRequest, CreateUserRequest, UpdateUserRequest, UserListResponse, UserResponse,
    },
    utils::{jwt::Claims, response::ApiResponse},
};
use actix_multipart::{Field, Multipart};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};

use futures_util::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;
use uuid::Uuid;
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
    post,
    path = "/api/users/create",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate email")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    // Validasi input
    request
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    let result = app_state.user_service.create(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(result)))
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
// backend/src/modules/users/handler.rs
#[utoipa::path(
    post,
    path = "/api/users/create_multipart",
    request_body(content_type = "multipart/form-data", content = CreateUserMultipartRequest),
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate email"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users",
    security(
        ("bearer_auth" = [])
    )
)]

pub async fn create_multipart(
    app_state: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let mut user_data = CreateUserMultipart::default();
    let mut roles = Vec::new();
    let mut image_path: Option<String> = None;

    // Iterasi melalui semua field multipart
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::BadRequest(e.to_string()))?;

        // Get field name dengan cara yang lebih aman
        let field_name = if let Some(name) = field.name() {
            name.to_string()
        } else {
            // Skip fields tanpa nama
            continue;
        };

        println!("Processing field: {}", field_name);

        // Handle file upload
        if field_name == "image" {
            // Process image
            let result = process_image_field(&mut field).await?;
            if let Some(path) = result {
                image_path = Some(path);
            }
            continue;
        }

        // Handle text fields
        let value = process_text_field_simple(&mut field).await?;

        // Map fields
        match field_name.as_str() {
            "name" => user_data.name = value,
            "email" => user_data.email = value,
            "password" => user_data.password = value,
            "dob" => user_data.dob = Some(value),
            "pob" => user_data.pob = Some(value),
            "phone" => user_data.phone = Some(value),
            "gender" => user_data.gender = Some(value),
            "address" => user_data.address = Some(value),
            "city" => user_data.city = Some(value),
            "province" => user_data.province = Some(value),
            "country" => user_data.country = Some(value),
            "postal_code" => user_data.postal_code = Some(value),
            "bio" => user_data.bio = Some(value),
            "status" => user_data.status = value,
            "latitude" => user_data.latitude = Some(value),
            "longitude" => user_data.longitude = Some(value),
            "timezone" => user_data.timezone = Some(value),
            "foundation_id" => {
                user_data.foundation_id = value
                    .parse()
                    .map_err(|_| AppError::BadRequest("Invalid foundation_id".to_string()))?;
            }
            field if field.starts_with("roles") => {
                roles.push(value);
            }
            _ => {
                // Log unknown fields but don't fail
                println!("Unknown field: {} = {}", field_name, value);
            }
        }
    }

    // Set image path
    user_data.image_path = image_path;

    // Set roles
    if !roles.is_empty() {
        user_data.roles = Some(roles);
    }

    // Debug data
    println!("Parsed user data: {:?}", user_data);
    match app_state
        .user_service
        .create_from_multipart(user_data)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("User Berhasil ditambahkan"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

async fn process_image_field(field: &mut Field) -> Result<Option<String>, AppError> {
    // Check if field actually contains a file
    let content_disposition = field.content_disposition();

    if content_disposition.is_none() {
        return Ok(None);
    }

    let filename = content_disposition.unwrap().get_filename();
    if filename.is_none() {
        return Ok(None);
    }

    let filename = filename.unwrap();
    println!("Uploading file: {}", filename);

    // Generate unique filename
    let new_filename = format!("{}_{}", Uuid::new_v4(), filename);

    let upload_dir = "uploads/users";
    fs::create_dir_all(upload_dir).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let filepath = format!("{}/{}", upload_dir, new_filename);
    let mut file =
        fs::File::create(&filepath).map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // Write file
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| AppError::BadRequest(e.to_string()))?;
        file.write_all(&data)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    }

    Ok(Some(filepath))
}

async fn process_text_field_simple(field: &mut Field) -> Result<String, AppError> {
    let mut bytes = Vec::new();

    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| AppError::BadRequest(e.to_string()))?;
        bytes.extend_from_slice(&data);
    }

    String::from_utf8(bytes).map_err(|e| AppError::BadRequest(e.to_string()))
}
