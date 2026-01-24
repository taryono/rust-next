// backend/src/modules/employees/dto.rs
use chrono::NaiveDate;
use entity::sea_orm_active_enums::EmploymentType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct EmployeeCountResponse {
    pub total: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmployeeResponse {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub foundation_id: i64,
    pub unit_id: Option<i64>,
    pub employee_number: String,
    pub nik: Option<String>,
    pub position_id: i32,
    pub department_id: i32,
    pub employment_type: Option<EmploymentType>,
    pub hire_date: Option<NaiveDate>,
    pub salary: Option<String>,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateEmployeeRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub user_id: i64,
    pub foundation_id: i64,
    pub unit_id: Option<i64>,
    pub employee_number: String,
    pub nik: Option<String>,
    pub position_id: i32,
    pub department_id: Option<i32>,
    pub employment_type: Option<EmploymentType>,
    pub hire_date: Option<NaiveDate>,
    pub salary: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateEmployeeRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub unit_id: Option<i64>,
    pub employee_number: String,
    pub nik: Option<String>,
    pub position_id: i32,
    pub department_id: Option<i32>,
    pub employment_type: Option<EmploymentType>,
    pub hire_date: Option<NaiveDate>,
    pub salary: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmployeeListResponse {
    pub data: Vec<EmployeeResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::employees::Model> for EmployeeResponse {
    fn from(model: entity::employees::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            user_id: model.user_id,
            unit_id: model.unit_id,
            employee_number: model.employee_number,
            nik: model.nik,
            position_id: model.position_id,
            department_id: model.department_id.unwrap_or_default(),
            employment_type: model.employment_type,
            hire_date: model
                .hire_date
                .is_some()
                .then(|| model.hire_date.unwrap().to_string().parse().unwrap()),
            salary: model.salary.map(|s| s.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl EmployeeResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::employees::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::employees::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmployeeFilters {
    // Pagination fields
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,

    // Known filters
    pub search: Option<String>,
    pub status: Option<String>,
    pub department: Option<String>,
    pub foundation: Option<String>,
    pub position: Option<String>,
    pub unit: Option<String>,

    // Catch-all untuk params lainnya
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}
