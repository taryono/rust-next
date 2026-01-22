// backend/src/modules/departments/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct DepartmentResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini (good practice)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDepartmentRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDepartmentRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DepartmentListResponse {
    pub data: Vec<DepartmentResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::departments::Model> for DepartmentResponse {
    fn from(model: entity::departments::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl DepartmentResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::departments::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::departments::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
