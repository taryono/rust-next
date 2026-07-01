// api/src/modules/academic/employees/handler.rs
// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateEmployeeRequest, EmployeeResponse, UpdateEmployeeRequest};
use crate::app_state::AppState;
use crate::context::ServiceContext; // ← import ServiceContext
use crate::errors::AppError;
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};
/// Create employee
#[utoipa::path(
    post,
    path = "/api/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee created successfully", body = EmployeeResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Conflict - duplicate name or overlapping dates")
    ),
    tag = "Employee "
)]
pub async fn create(
    app_state: web::Data<AppState>,
    request: web::Json<CreateEmployeeRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .employee_service
        .create(request.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(result))
}

/// Get employee by ID
#[utoipa::path(
    get,
    path = "/api/employees/{id}",
    params(
        ("id" = i64, Path, description = "Employee ID")
    ),
    responses(
        (status = 200, description = "Employee found", body = EmployeeResponse),
        (status = 404, description = "Employee not found")
    ),
    tag = "Employee "
)]
pub async fn get_by_id(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .employee_service
        .get_by_id(id.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Get all employees with pagination
#[utoipa::path(
    get,
    path = "/api/employees",
    params(
        ("page" = Option<i64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<i64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort field"),
        ("sort_order" = Option<String>, Query, description = "Sort order: asc or desc (default: desc)"),
    ),
    responses(
        (status = 200, description = "List of employees", body = PaginatedResponse<EmployeeResponse>)
    ),
    tag = "Employee "
)]
pub async fn get_all(
    app_state: web::Data<AppState>,
    query: web::Query<PaginationParams>,
    ctx: ServiceContext,
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    let foundation_id = ctx.foundation_id;
    // Untuk admin (semua foundation)
    print!("params: {:#?}\n", params);
    print!("Foundation ID: {}\n", foundation_id);

    match app_state
        .employee_service
        .get_all(params, Some(foundation_id))
        .await
    {
        Ok(roles) => Ok(HttpResponse::Ok().json(ApiResponse::success(roles))),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string())))
        }
    }
}

/// Update employee
#[utoipa::path(
    put,
    path = "/api/employees/{id}",
    params(
        ("id" = i64, Path, description = "Employee ID")
    ),
    request_body = UpdateEmployeeRequest,
    responses(
        (status = 200, description = "Employee updated", body = EmployeeResponse),
        (status = 404, description = "Employee not found"),
        (status = 409, description = "Conflict")
    ),
    tag = "Employee "
)]
pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
    request: web::Json<UpdateEmployeeRequest>,
) -> Result<HttpResponse, AppError> {
    let result = app_state
        .employee_service
        .update(id.into_inner(), request.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

/// Delete employee
#[utoipa::path(
    delete,
    path = "/api/employees/{id}",
    params(
        ("id" = i64, Path, description = "Employee ID")
    ),
    responses(
        (status = 204, description = "Employee deleted"),
        (status = 404, description = "Employee not found")
    ),
    tag = "Employee "
)]
pub async fn delete(
    app_state: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    app_state.employee_service.delete(id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
