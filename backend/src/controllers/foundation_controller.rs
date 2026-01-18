// ============================================
// backend/src/controllers/foundation_controller.rs
// ============================================
use crate::models::pagination::PaginationParams;
use crate::{
    config::database::Database,
    models::foundation::{FoundationListResponse, FoundationResponse, UpdateFoundationRequest},
    services::foundation_service,
    utils::{jwt::Claims, response::ApiResponse},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/foundations",
    tag = "foundations",
    params(
        ("page" = Option<i64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<i64>, Query, description = "Items per page, default 10, max 100"),
        ("search" = Option<String>, Query, description = "Search by name or description"),
    ),
    responses(
        (status = 200, description = "List of foundations retrieved successfully", body = FoundationListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_foundations(
    db: web::Data<Database>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    match foundation_service::get_all_foundations(db.get_connection(), query.into_inner()).await {
        Ok(foundations) => HttpResponse::Ok().json(ApiResponse::success(foundations)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/foundations/{id}",
    tag = "foundations",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 200, description = "Foundation found", body = FoundationResponse),
        (status = 404, description = "Foundation not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_foundation_by_id(db: web::Data<Database>, path: web::Path<i64>) -> HttpResponse {
    let foundation_id = path.into_inner();

    match foundation_service::get_foundation_by_id(db.get_connection(), foundation_id).await {
        Ok(foundation) => HttpResponse::Ok().json(ApiResponse::success(foundation)),
        Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/api/foundations/me",
    tag = "foundations",
    responses(
        (status = 200, description = "Current foundation profile", body = FoundationResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Foundation not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_foundation(db: web::Data<Database>, req: HttpRequest) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(claims) => {
            let foundation_id: i64 = claims.sub.parse().unwrap_or(0);

            match foundation_service::get_foundation_by_id(db.get_connection(), foundation_id).await
            {
                Ok(foundation) => HttpResponse::Ok().json(ApiResponse::success(foundation)),
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
    path = "/api/foundations/me",
    tag = "foundations",
    request_body = UpdateFoundationRequest,
    responses(
        (status = 200, description = "Foundation updated successfully", body = FoundationResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_foundation(
    db: web::Data<Database>,
    req: HttpRequest,
    body: web::Json<UpdateFoundationRequest>,
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
            let foundation_id: i64 = claims.sub.parse().unwrap_or(0);

            match foundation_service::update_foundation(
                db.get_connection(),
                foundation_id,
                body.into_inner(),
            )
            .await
            {
                Ok(foundation) => HttpResponse::Ok().json(ApiResponse::success(foundation)),
                Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
            }
        }
        None => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error("Unauthorized".to_string()))
        }
    }
}

/// Soft delete foundation (replaces delete_foundation)
#[utoipa::path(
    delete,
    path = "/api/foundations/{id}",
    tag = "foundations",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 200, description = "Foundation soft deleted successfully"),
        (status = 404, description = "Foundation not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_foundation(db: web::Data<Database>, path: web::Path<i64>) -> HttpResponse {
    let foundation_id = path.into_inner();

    match foundation_service::soft_delete(db.get_connection(), foundation_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Foundation deleted successfully")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Restore deleted foundation
#[utoipa::path(
    post,
    path = "/api/foundations/{id}/restore",
    tag = "foundations",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 200, description = "Foundation restored successfully"),
        (status = 404, description = "Foundation not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn restore_foundation(db: web::Data<Database>, path: web::Path<i64>) -> HttpResponse {
    let foundation_id = path.into_inner();

    match foundation_service::restore(db.get_connection(), foundation_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Foundation restored successfully")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Force delete foundation (permanent)
#[utoipa::path(
    delete,
    path = "/api/foundations/{id}/force",
    tag = "foundations",
    params(
        ("id" = i64, Path, description = "Foundation ID")
    ),
    responses(
        (status = 200, description = "Foundation permanently deleted"),
        (status = 404, description = "Foundation not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn force_delete_foundation(
    db: web::Data<Database>,
    path: web::Path<i64>,
) -> HttpResponse {
    let foundation_id = path.into_inner();

    match foundation_service::force_delete(db.get_connection(), foundation_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Foundation permanently deleted")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string())),
    }
}

/// Get deleted foundations
#[utoipa::path(
    get,
    path = "/api/foundations/deleted",
    tag = "foundations",
    params(
        ("page" = Option<i64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<i64>, Query, description = "Items per page, default 10, max 100"),
    ),
    responses(
        (status = 200, description = "List of deleted foundations"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_deleted_foundations(
    db: web::Data<Database>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    match foundation_service::get_deleted_foundations(db.get_connection(), query.into_inner()).await
    {
        Ok(foundations) => HttpResponse::Ok().json(ApiResponse::success(foundations)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())),
    }
}
