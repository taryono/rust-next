// api/src/modules/institution/sport/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct SportResponse {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSportRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSportRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SportListResponse {
    pub data: Vec<SportResponse>,
    pub total: i64,
    pub page: u64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::sports::Model> for SportResponse {
    fn from(model: entity::sports::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl SportResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::sports::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::sports::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
