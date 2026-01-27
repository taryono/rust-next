// backend/src/modules/settings/dto.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct SettingResponse {
    pub foundation_id: i64,
    pub name: String,
    pub academic_year_id: i64,
    pub unit_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_active: bool,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSettingRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub academic_year_id: i64,
    pub unit_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_active: bool,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSettingRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub academic_year_id: i64,
    pub unit_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_active: bool,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}
#[derive(Debug, Serialize, ToSchema)]
pub struct SettingListResponse {
    pub data: Vec<SettingResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::settings::Model> for SettingResponse {
    fn from(model: entity::settings::Model) -> Self {
        Self {
            foundation_id: model.foundation_id,
            name: model.name,
            academic_year_id: model.academic_year_id,
            unit_id: model.unit_id,
            start_date: model.start_date,
            end_date: model.end_date,
            is_active: model.is_active.to_string() == "1",
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl SettingResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::settings::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::settings::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
