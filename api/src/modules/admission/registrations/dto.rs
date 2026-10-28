// api/src/modules/admission/registrations/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct RegistrationResponse {
    pub id: i64,
    pub code: String,
    pub foundation_id: i64,
    pub name: String,
    pub address: Option<String>,
    pub config_schema: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRegistrationRequest {
    pub foundation_id: i64,
    pub code: String,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub address: Option<String>,
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateRegistrationRequest {
    pub code: Option<String>,
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    pub address: Option<String>,
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegistrationListResponse {
    pub data: Vec<RegistrationResponse>,
    pub total: i64,
    pub page: u64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl From<entity::registrations::Model> for RegistrationResponse {
    fn from(model: entity::registrations::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            foundation_id: model.foundation_id,
            name: model.name,
            address: model.address,
            config_schema: model.config_schema,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

impl RegistrationResponse {
    pub fn from_entity(model: entity::registrations::Model) -> Self {
        Self::from(model)
    }
    pub fn from_vec(dto: Vec<entity::registrations::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}
