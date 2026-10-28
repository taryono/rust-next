// api/src/modules/finance/invoices/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub config_schema: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceListResponse {
    pub data: Vec<InvoiceResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInvoiceRequest {
    #[validate(length(min = 1, max = 50))]
    pub code: String,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub description: Option<String>,
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateInvoiceRequest {
    #[validate(length(min = 1, max = 50))]
    pub code: Option<String>,

    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub description: Option<String>,
    pub config_schema: Option<serde_json::Value>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::invoices::Model> for InvoiceResponse {
    fn from(model: entity::invoices::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            description: model.description,
            config_schema: model.config_schema,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl InvoiceResponse {
    pub fn from_vec(models: Vec<entity::invoices::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
