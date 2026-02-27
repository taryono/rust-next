// backend/src/modules/units/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct UnitResponse {
    pub id: i64,
    pub name: String,
    pub foundation_id: i64,
    pub unit_type_id: Option<String>,
    pub class_level_id: Option<i32>,
    pub level_id: i64,
    pub parent_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUnitRequest {
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UnitOptionResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUnitRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    pub foundation_id: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UnitListResponse {
    pub data: Vec<UnitResponse>,
    pub total: i64,
    pub page: u64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::units::Model> for UnitResponse {
    fn from(model: entity::units::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            unit_type_id: model.unit_type_id,
            class_level_id: model.class_level_id,
            level_id: model.level_id,
            parent_id: model.parent_id,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
impl UnitResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::units::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::units::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}

impl From<entity::units::Model> for UnitOptionResponse {
    fn from(model: entity::units::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

impl UnitOptionResponse {
    pub fn from_vec(dto: Vec<entity::units::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
