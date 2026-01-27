// backend/src/modules/semesters/dto.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Debug, Serialize, ToSchema)]
pub struct SemesterResponse {
    pub foundation_id: i64,
    pub academic_calendar_id: i64,
    pub semester_number: i8,
    pub year: i32,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub name: String,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSemesterRequest {
    pub foundation_id: i64,
    pub academic_calendar_id: i64,
    pub semester_number: i8,
    pub year: i32,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSemesterRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub academic_calendar_id: i64,
    pub semester_number: i8,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_active: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SemesterListResponse {
    pub data: Vec<SemesterResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::semesters::Model> for SemesterResponse {
    fn from(model: entity::semesters::Model) -> Self {
        Self {
            foundation_id: model.foundation_id,
            name: model.name,
            academic_calendar_id: model.academic_calendar_id,
            semester_number: model.semester_number,
            year: model.year,
            start_date: model.start_date.to_string(),
            end_date: model.end_date.to_string(),
            is_active: model.is_active.to_string() == "1",
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl SemesterResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::semesters::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::semesters::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
