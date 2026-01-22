// backend/src/modules/foundations/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationResponse {
    pub id: i64,
    pub name: String,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateFoundationRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateFoundationRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationListResponse {
    pub data: Vec<FoundationResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::foundations::Model> for FoundationResponse {
    fn from(model: entity::foundations::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl FoundationResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::foundations::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::foundations::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
