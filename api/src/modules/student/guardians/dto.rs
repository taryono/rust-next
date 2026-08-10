// api/src/modules/guardians/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct GuardianResponse {
    pub id: i64,
    pub user_id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub address: String,
    pub cellphone: Option<String>,
    pub salary: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GuardianListResponse {
    pub data: Vec<GuardianResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateGuardianRequest {
    pub user_id: i64,
    pub foundation_id: i64,
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(min = 3, max = 100))]
    pub address: String,
    pub cellphone: Option<String>,
    pub salary: Option<i64>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateGuardianRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 3, max = 100))]
    pub address: Option<String>,
    pub cellphone: Option<String>,
    pub salary: Option<i64>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::guardians::Model> for GuardianResponse {
    fn from(model: entity::guardians::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            foundation_id: model.foundation_id,
            name: model.name,
            address: model.address,
            cellphone: Some(model.cellphone),
            salary: model.salary,
            created_at: model.created_at.to_rfc3339(), // ✅ format ISO 8601
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl GuardianResponse {
    pub fn from_vec(models: Vec<entity::guardians::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
