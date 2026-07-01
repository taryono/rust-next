// api/src/modules/institution/school/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct SchoolResponse {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String, // ← Tambah ini
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSchoolRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateSchoolRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SchoolListResponse {
    pub data: Vec<SchoolResponse>,
    pub total: i64,
    pub page: u64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::schools::Model> for SchoolResponse {
    fn from(model: entity::schools::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl SchoolResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::schools::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::schools::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
