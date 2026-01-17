use actix_web::{web, HttpResponse};
use validator::Validate;

use crate::{
    config::database::Database,
    models::{
        academic_year::{
            AcademicYearListResponse, AcademicYearResponse, CreateAcademicYearRequest,
            UpdateAcademicYearRequest,
        },
        pagination::PaginationParams,
    },
    services::academic_year_service,
    utils::response::ApiResponse,
};

#[utoipa::path(
    get,
   path = "/api/academic-years",
    tag = "academic-years",
    params(
        ("page" = Option<u64>, Query, description = "Page number, default 1"),
        ("per_page" = Option<u64>, Query, description = "Items per page, default 10, max 100"),
        ("search" = Option<String>, Query, description = "Search by name or year"), 
    ),
    responses(
        (status = 200, description = "List of users retrieved successfully", body = AcademicYearListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_all(db: web::Data<Database>, query: web::Query<PaginationParams>) -> HttpResponse {
    match academic_year_service::get_all(db.get_connection(), query.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/academic-years/{id}",
    tag = "academic-years",
    params(("id" = u64, Path)), 
    responses(
        (status = 200, description = "List of academic year retrieved successfully", body = AcademicYearResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_by_id(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    match academic_year_service::get_by_id(db.get_connection(), path.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
        Err(e) => HttpResponse::NotFound().json(ApiResponse::<()>::error(e)),
    }
}

#[utoipa::path(
    post,
    path = "/api/academic-years",
    tag = "academic-years",
    request_body = CreateAcademicYearRequest,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create(
    db: web::Data<Database>,
    body: web::Json<CreateAcademicYearRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()));
    }

    match academic_year_service::create(db.get_connection(), body.into_inner()).await {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

#[utoipa::path(
    put,
    path = "/api/academic-years/{id}",
    tag = "academic-years",
    request_body = UpdateAcademicYearRequest,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update(
    db: web::Data<Database>,
    path: web::Path<u64>,
    body: web::Json<UpdateAcademicYearRequest>,
) -> HttpResponse {
    match academic_year_service::update(db.get_connection(), path.into_inner(), body.into_inner())
        .await
    {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}

#[utoipa::path(
    delete, 
    path = "/api/academic-years/{id}", 
    tag = "academic-years",
    security(
        ("bearer_auth" = [])
    ))]
pub async fn delete(db: web::Data<Database>, path: web::Path<u64>) -> HttpResponse {
    match academic_year_service::delete(db.get_connection(), path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Deleted")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::<()>::error(e)),
    }
}
