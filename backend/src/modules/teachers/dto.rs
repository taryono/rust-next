// backend/src/modules/teachers/dto.rs
use chrono::NaiveDate;
use entity::sea_orm_active_enums::EmploymentStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Debug, Serialize, ToSchema)]
pub struct TeacherResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub user_id: i64,
    pub unit_id: Option<i64>,
    pub nik: Option<String>,
    pub employee_number: String,
    pub specialization: Option<String>,
    pub qualification: Option<String>,
    pub hire_date: Option<String>, // ← Tambah ini (good practice)
    pub end_date: Option<String>,  // ← Tambah ini (good practice)
    pub salary: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTeacherRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub user_id: i64,
    pub unit_id: Option<i64>,
    pub nik: Option<String>,
    pub employee_number: String,
    pub specialization: Option<String>,
    pub qualification: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub salary: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
    pub end_date: Option<NaiveDate>, // ← Tambah ini (good practice)
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTeacherRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub user_id: i64,
    pub foundation_id: i64,
    pub unit_id: Option<i64>,
    pub nik: Option<String>,
    pub employee_number: String,
    pub specialization: Option<String>,
    pub qualification: Option<String>,
    pub hire_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>, // ← Tambah ini (good practice)
    pub salary: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TeacherListResponse {
    pub data: Vec<TeacherResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::teachers::Model> for TeacherResponse {
    fn from(model: entity::teachers::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            user_id: model.user_id,
            unit_id: model.unit_id,
            nik: model.nik,
            employee_number: model.employee_number,
            specialization: model.specialization,
            qualification: model.qualification,
            hire_date: model.hire_date.map(|hd| hd.to_string().parse().unwrap()),
            end_date: model.end_date.map(|ed| ed.to_string().parse().unwrap()),
            salary: model.salary,
            employment_status: model.employment_status,
            created_at: model.created_at.to_string(), // jika tipe data string maka harus di casting ke string
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl TeacherResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::teachers::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::teachers::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
