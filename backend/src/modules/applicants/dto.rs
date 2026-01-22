// backend/src/modules/applicants/dto.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Debug, Serialize, ToSchema)]
pub struct ApplicantResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub birth_place: String,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini (good practice)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateApplicantRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub birth_place: String,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini (good practice)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateApplicantRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub birth_place: String,
    pub birth_date: NaiveDate,
    pub gender: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini (good practice)
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApplicantListResponse {
    pub data: Vec<ApplicantResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::applicants::Model> for ApplicantResponse {
    fn from(model: entity::applicants::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            birth_place: model.birth_place,
            birth_date: model.birth_date,
            gender: model.gender,
            email: model.email,
            phone: model.phone,
            address: model.address,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl ApplicantResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::applicants::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::applicants::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
