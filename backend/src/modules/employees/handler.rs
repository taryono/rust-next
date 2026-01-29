// backend/src/modules/employees/handler.rs
// ============================================================================
// handler.rs - HTTP Handlers
// ============================================================================
use super::dto::{CreateEmployeeRequest, EmployeeFilters, EmployeeResponse, UpdateEmployeeRequest};
use crate::app_state::AppState;
use crate::errors::AppError;
use crate::middleware::auth::AuthContext; // ✅ Import AuthContext
use crate::utils::{
    pagination::{PaginatedResponse, PaginationParams},
    response::ApiResponse,
};
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
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
    filters: web::Query<EmployeeFilters>,
    // Optional: foundation_id dari auth/context
    auth: web::ReqData<AuthContext>, // ✅ Extract AuthContext, // ✅ Extract full Claims
) -> Result<HttpResponse, AppError> {
    let params = query.into_inner();
    let foundation_id = auth.foundation_id;
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

pub async fn get_all_with_dynamic_params(
    app_state: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>, // <-- ini jika parameter dinamis tanpa membuat struct untuk parameter
) -> Result<HttpResponse, AppError> {
    let page = query
        .get("page")
        .and_then(|p| p.parse::<u64>().ok())
        .unwrap_or(1);

    let per_page = query
        .get("per_page")
        .and_then(|p| p.parse::<u64>().ok())
        .unwrap_or(10);

    let status = query.get("status").cloned();
    let sort_order = query.get("sort_order").cloned();

    // ... rest of your logic

    Ok(HttpResponse::Ok().json(()))
}
