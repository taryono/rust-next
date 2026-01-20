// backend/src/modules/academic_years/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademicYearResponse {
    pub id: u64,
    pub foundation_id: u64,
    pub name: String,
    pub start_date: String, // ✅ Correct - akan di-convert dari Date
    pub end_date: String,   // ✅ Correct
    pub is_active: i8,      // ✅ Correct - NOT NULL di DB
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAcademicYearRequest {
    pub foundation_id: u64,

    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[validate(length(min = 10, max = 10))] // Format: YYYY-MM-DD
    pub start_date: String,

    #[validate(length(min = 10, max = 10))]
    pub end_date: String,

    #[validate(range(min = 0, max = 1))]
    pub is_active: Option<i8>, // Optional saat create, default 0
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAcademicYearRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[validate(length(min = 10, max = 10))]
    pub start_date: Option<String>,

    #[validate(length(min = 10, max = 10))]
    pub end_date: Option<String>,

    #[validate(range(min = 0, max = 1))]
    pub is_active: Option<i8>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademicYearListResponse {
    pub data: Vec<AcademicYearResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::academic_years::Model> for AcademicYearResponse {
    fn from(model: entity::academic_years::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            start_date: model.start_date.to_string(),
            end_date: model.end_date.to_string(),
            is_active: model.is_active.into(),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl AcademicYearResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::academic_years::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(models: Vec<entity::academic_years::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
