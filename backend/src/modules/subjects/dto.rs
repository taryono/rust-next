// backend/src/modules/subjects/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct SubjectResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSubjectRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSubjectRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    pub foundation_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubjectListResponse {
    pub data: Vec<SubjectResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::subjects::Model> for SubjectResponse {
    fn from(model: entity::subjects::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl SubjectResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::subjects::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::subjects::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
