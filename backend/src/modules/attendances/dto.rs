// backend/src/modules/attendances/dto.rs
use chrono::NaiveDate;
use entity::sea_orm_active_enums::Status;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Debug, Serialize, ToSchema)]
pub struct AttendanceResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub student_id: i64,
    pub class_subject_id: i64,
    pub date: NaiveDate,
    pub status: Status,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini (good practice)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAttendanceRequest {
    pub foundation_id: i64,
    pub student_id: i64,
    pub class_subject_id: i64,
    pub date: NaiveDate,
    pub status: Status,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAttendanceRequest {
    pub foundation_id: i64,
    pub student_id: i64,
    pub class_subject_id: i64,
    pub date: NaiveDate,
    pub status: Status,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AttendanceListResponse {
    pub data: Vec<AttendanceResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::attendances::Model> for AttendanceResponse {
    fn from(model: entity::attendances::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            student_id: model.student_id,
            class_subject_id: model.class_subject_id,
            date: model.date.to_string().parse().unwrap(),
            status: model.status,
            notes: model.notes,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl AttendanceResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::attendances::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::attendances::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
