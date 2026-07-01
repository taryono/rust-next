// ============================================================================
// api/src/modules/users/handler.rs
// ============================================================================
// use super::dto_multipart::{CreateUserMultipart, CreateUserMultipartRequest};
// use crate::context::ServiceContext;
// use crate::utils::pagination::PaginationParams;
// use crate::{
//     app_state::AppState,
//     errors::AppError,
//     modules::identity::users::dto::{
//         AssignRoleRequest, ChangePasswordRequest, CreateUserRequest, SyncRolesRequest,
//         UpdateUserRequest, UserListResponse, UserResponse,
//     },
//     utils::response::ApiResponse,
// };
// use actix_multipart::{Field, Multipart};
// use actix_web::{web, HttpResponse, Result};
// use futures_util::StreamExt;
// use std::fs;
// use std::io::Write;
// use tokio::io::AsyncWriteExt;
// use uuid::Uuid;
// use validator::Validate;
use super::dto_multipart::{CreateUserMultipart, CreateUserMultipartRequest};
use crate::context::ServiceContext;
use crate::utils::pagination::PaginationParams;
use crate::{
    app_state::AppState,
    errors::AppError,
    modules::identity::users::dto::{
        AssignRoleRequest, ChangePasswordRequest, CreateUserRequest, SyncRolesRequest,
        UpdateUserRequest, UserListResponse, UserResponse,
    },
    utils::response::ApiResponse,
};
use actix_multipart::{Field, Multipart};
use actix_web::{web, HttpResponse, Result};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt; // ✅ hanya tokio, hapus std::fs & std::io::Write
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// USER CRUD
// ============================================================================

#[utoipa::path(
    get,
    path = "/api/users",
    tag = "Users",
    params(
        ("page" = Option<i64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<i64>, Query, description = "Items per page, default 10, max 100"),
        ("search" = Option<String>, Query, description = "Search by name or email"),
    ),
    responses(
        (status = 200, description = "List of users retrieved successfully", body = UserListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_users(
    app_state: web::Data<AppState>,
    ctx: ServiceContext,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    let users = app_state
        .user_service
        .get_all(&ctx, query.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(users)))
}

#[utoipa::path(
    post,
    path = "/api/users",
    tag = "Users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create(
    app_state: web::Data<AppState>,
    ctx: ServiceContext,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    // ✅ Validasi hanya di handler, tidak perlu diulang di service
    request
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let result = app_state
        .user_service
        .create(&ctx, request.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse::success(result)))
}

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let user = app_state.user_service.get_by_id(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Users",
    responses(
        (status = 200, description = "Current user profile", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_current_user(
    app_state: web::Data<AppState>,
    ctx: ServiceContext, // ✅ Pakai ServiceContext, bukan manual Claims
) -> Result<HttpResponse, AppError> {
    let user = app_state.user_service.get_by_id(ctx.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

#[utoipa::path(
    put,
    path = "/api/users/me",
    tag = "Users",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update(
    app_state: web::Data<AppState>,
    ctx: ServiceContext, // ✅ Pakai ServiceContext
    body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = app_state
        .user_service
        .update(ctx.user_id, body.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

#[utoipa::path(
    post,
    path = "/api/users/change-password",
    tag = "Users",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Validation error or incorrect old password"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn change_password(
    app_state: web::Data<AppState>,
    ctx: ServiceContext, // ✅ Pakai ServiceContext
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    app_state
        .user_service
        .change_password(ctx.user_id, body.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Password changed successfully")))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
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
) -> Result<HttpResponse, AppError> {
    app_state
        .user_service
        .soft_delete(path.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("User deleted successfully")))
}

#[utoipa::path(
    post,
    path = "/api/users/{id}/restore",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User restored successfully", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_user(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    // ✅ Return UserResponse yang di-restore, bukan diabaikan
    let user = app_state.user_service.restore(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}/force",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
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
) -> Result<HttpResponse, AppError> {
    app_state.user_service.delete(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success("User permanently deleted")))
}

// ============================================================================
// ROLE MANAGEMENT
// ============================================================================

#[utoipa::path(
    get,
    path = "/api/users/{id}/roles",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User with roles", body = UserResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_user_with_roles(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let user = app_state
        .user_service
        .get_with_roles(path.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(user)))
}

#[utoipa::path(
    post,
    path = "/api/users/{id}/roles",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
    request_body = AssignRoleRequest,
    responses(
        (status = 200, description = "Role assigned successfully"),
        (status = 404, description = "User or role not found"),
        (status = 409, description = "User already has this role"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_role(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
    body: web::Json<AssignRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    app_state
        .user_service
        .assign_role(user_id, body.role_id)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Role assigned successfully")))
}

#[utoipa::path(
    delete,
    path = "/api/users/{id}/roles/{role_id}",
    tag = "Users",
    params(
        ("id" = i64, Path, description = "User ID"),
        ("role_id" = i64, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role removed successfully"),
        (status = 404, description = "Role assignment not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn remove_role(
    app_state: web::Data<AppState>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse, AppError> {
    let (user_id, role_id) = path.into_inner();

    app_state.user_service.remove_role(user_id, role_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Role removed successfully")))
}

#[utoipa::path(
    put,
    path = "/api/users/{id}/roles",
    tag = "Users",
    params(("id" = i64, Path, description = "User ID")),
    request_body = SyncRolesRequest,
    responses(
        (status = 200, description = "Roles synced successfully"),
        (status = 404, description = "User or role not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn sync_roles(
    app_state: web::Data<AppState>,
    path: web::Path<i64>,
    body: web::Json<SyncRolesRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    app_state
        .user_service
        .sync_roles(user_id, body.into_inner().role_ids)
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Roles synced successfully")))
}

// ============================================================================
// MULTIPART
// ============================================================================

#[utoipa::path(
    post,
    path = "/api/users/multipart",
    tag = "Users",
    request_body(content_type = "multipart/form-data", content = CreateUserMultipartRequest),
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Email already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
// ============================================================================
// MULTIPART
// ============================================================================

pub async fn create_multipart(
    app_state: web::Data<AppState>,
    ctx: ServiceContext,
    mut payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let mut user_data = CreateUserMultipart::default();
    let mut roles: Vec<String> = Vec::new();
    let mut saved_image_path: Option<String> = None; // ✅ Fix #1: None bukan null

    // ✅ Fix #2: Satu while loop saja, tidak ada duplikasi
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::BadRequest(e.to_string()))?;

        let field_name = match field.name() {
            Some(name) => name.to_string(),
            None => continue,
        };

        if field_name == "image" {
            // ✅ Fix #3: simpan ke variabel terpisah dulu
            // ✅ upload_dir dari config, bukan hardcoded
            let upload_dir = app_state.config.upload_dir_for("users");
            let path = process_image_field(&mut field, &upload_dir, &app_state).await?;
            user_data.image_path = path.clone();
            saved_image_path = path; // untuk keperluan cleanup
            continue;
        }

        let value = process_text_field(&mut field).await?;

        match field_name.as_str() {
            "name" => user_data.name = value,
            "email" => user_data.email = value,
            "password" => user_data.password = value,
            "status" => user_data.status = value,
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
            "latitude" => user_data.latitude = Some(value),
            "longitude" => user_data.longitude = Some(value),
            "timezone" => user_data.timezone = Some(value),
            "foundation_id" => {
                user_data.foundation_id = value
                    .parse()
                    .map_err(|_| AppError::BadRequest("Invalid foundation_id".to_string()))?;
            }
            "roles[]" | "roles" => roles.push(value),
            unknown => tracing::warn!("Unknown multipart field: {}", unknown),
        }
    }

    if !roles.is_empty() {
        user_data.roles = Some(roles);
    }

    user_data
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // ✅ Fix #4: cleanup orphaned image jika service gagal
    let result = app_state
        .user_service
        .create_from_multipart(user_data)
        .await
        .map_err(|e| {
            if let Some(ref path) = saved_image_path {
                let path = path.clone();
                tokio::spawn(async move {
                    let _ = tokio::fs::remove_file(path).await;
                });
            }
            e
        })?;

    Ok(HttpResponse::Created().json(ApiResponse::success(result)))
}

async fn process_image_field(
    field: &mut Field,
    upload_dir: &str,
    app_state: &AppState,
) -> Result<Option<String>, AppError> {
    let max_size = app_state.config.max_upload_size;

    let allowed_ext = &app_state.config.allowed_image_extensions;

    let filename = field
        .content_disposition()
        .and_then(|cd| cd.get_filename())
        .map(|f| f.to_string());

    // Jika tidak ada filename atau kosong, skip
    let filename = match filename {
        Some(f) if !f.is_empty() => f,
        _ => return Ok(None),
    };

    // ✅ SESUDAH
    let ext = filename.as_str(); // ← jelas &str

    if !allowed_ext.iter().any(|e| e == ext) {
        // ← iter().any() untuk Vec<String> vs &str
        return Err(AppError::BadRequest(format!(
            "Format tidak didukung. Gunakan: {}",
            allowed_ext.join(", ")
        )));
    }

    // ✅ Fix #6: tokio::fs (non-blocking), bukan std::fs
    let upload_dir = "uploads/users";
    tokio::fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let filepath = format!("{}/{}_{}", upload_dir, Uuid::new_v4(), filename);
    let mut file = tokio::fs::File::create(&filepath)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // ✅ Fix #7: validasi ukuran file saat streaming
    let mut total_size = 0usize;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| AppError::BadRequest(e.to_string()))?;
        total_size += data.len();

        if total_size > max_size {
            // Cleanup file yang terlanjur dibuat
            let _ = tokio::fs::remove_file(&filepath).await;
            return Err(AppError::BadRequest(
                "Ukuran gambar maksimal 5MB".to_string(),
            ));
        }

        // ✅ Fix #8: write_all async, error di-map bukan pakai ?
        file.write_all(&data)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    }

    Ok(Some(filepath))
}

async fn process_text_field(field: &mut Field) -> Result<String, AppError> {
    let mut bytes = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| AppError::BadRequest(e.to_string()))?;
        bytes.extend_from_slice(&data);
    }
    String::from_utf8(bytes).map_err(|e| AppError::BadRequest(e.to_string()))
}

async fn process_json_field(field: &mut Field) -> Result<String, AppError> {
    process_text_field(field).await
}
