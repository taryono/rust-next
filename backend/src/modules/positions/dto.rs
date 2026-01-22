// backend/src/modules/positions/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct PositionResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePositionRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePositionRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PositionListResponse {
    pub data: Vec<PositionResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::positions::Model> for PositionResponse {
    fn from(model: entity::positions::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl PositionResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::positions::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::positions::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
