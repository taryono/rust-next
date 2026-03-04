// backend/src/modules/foundation_regulations/dto.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

// ============================================================================
// REQUEST
// ============================================================================

/// Dipakai saat pertama kali assign regulasi ke yayasan
/// foundation_code & regulation_code dari path param, bukan body
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateFoundationRegulationRequest {
    pub is_active: bool,
    pub config: Option<Value>,
}

/// Toggle aktif/nonaktif
#[derive(Debug, Deserialize, ToSchema)]
pub struct ToggleFoundationRegulationRequest {
    pub is_active: bool,
}

/// Update config saja
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateFoundationRegulationConfigRequest {
    pub config: Value,
}

// ============================================================================
// RESPONSE
// ============================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationRegulationResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub regulation_id: i64,
    pub regulation_code: String, // dari JOIN regulations
    pub regulation_name: String, // dari JOIN regulations
    pub is_active: bool,
    pub config: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

// ============================================================================
// LIST RESPONSE
// ============================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationRegulationListResponse {
    pub data: Vec<FoundationRegulationResponse>,
    pub total: i64,
    pub page: u64,
    pub per_page: i64,
    pub total_pages: i64,
}

// ============================================================================
// FROM TRAIT
// ============================================================================

impl From<entity::foundation_regulations::Model> for FoundationRegulationResponse {
    fn from(model: entity::foundation_regulations::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            regulation_id: model.regulation_id,
            regulation_code: String::new(), // diisi dari JOIN di repository
            regulation_name: String::new(), // diisi dari JOIN di repository
            is_active: model.is_active.unwrap_or(0) != 0,
            config: model.config,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            deleted_at: model.deleted_at.map(|d| d.to_string()),
        }
    }
}

impl FoundationRegulationResponse {
    pub fn from_vec(models: Vec<entity::foundation_regulations::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
