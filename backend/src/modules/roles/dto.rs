// backend/src/modules/roles/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub description: Option<String>,
    pub name: String,
    pub created_at: String, // ← Tambah ini (good practice)
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRoleRequest {
    pub foundation_id: i64,
    pub description: Option<String>,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleListResponse {
    pub data: Vec<RoleResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::roles::Model> for RoleResponse {
    fn from(model: entity::roles::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            description: model.description,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl RoleResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::roles::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::roles::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
