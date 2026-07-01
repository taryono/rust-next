// api/src/modules/academic/regulations/dto.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

// ============================================================================
// MASTER REGULATION DTO
// ============================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct RegulationResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub config_schema: Option<Value>, // JSON schema untuk validasi config
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRegulationRequest {
    #[validate(length(min = 3, max = 100))]
    pub code: String, // e.g. "PAYMENT_INSTALLMENT"
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub config_schema: Option<Value>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateRegulationRequest {
    #[validate(length(min = 3, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub config_schema: Option<Value>,
}

// ============================================================================
// FOUNDATION REGULATION DTO (regulasi per yayasan)
// ============================================================================

/// Response untuk regulasi yang sudah dikonfigurasi per yayasan
#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationRegulationResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub regulation_id: i64,
    pub regulation_code: String,
    pub regulation_name: String,
    pub is_active: bool,
    pub config: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

/// Toggle aktif/nonaktif regulasi untuk yayasan tertentu
#[derive(Debug, Deserialize, ToSchema)]
pub struct ToggleRegulationRequest {
    pub is_active: bool,
}

/// Update config regulasi untuk yayasan tertentu
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateFoundationRegulationConfigRequest {
    pub config: Value, // { "mode": "fixed", "fixed_terms": 3 }
}

// ============================================================================
// FROM TRAIT
// ============================================================================

impl From<entity::regulations::Model> for RegulationResponse {
    fn from(model: entity::regulations::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            description: model.description,
            config_schema: model.config_schema,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

impl RegulationResponse {
    pub fn from_vec(models: Vec<entity::regulations::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}

impl From<entity::foundation_regulations::Model> for FoundationRegulationResponse {
    fn from(model: entity::foundation_regulations::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            regulation_id: model.regulation_id,
            regulation_code: String::new(), // diisi dari JOIN di service
            regulation_name: String::new(), // diisi dari JOIN di service
            is_active: model.is_active.is_some(),
            config: model.config,
            created_at: model.created_at.to_string(),
            deleted_at: model.deleted_at.map(|d| d.to_string()),
            updated_at: model.updated_at.to_string(),
        }
    }
}
