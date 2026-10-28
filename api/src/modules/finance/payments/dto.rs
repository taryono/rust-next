// api/src/modules/finance/payments/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentListResponse {
    pub data: Vec<PaymentResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePaymentRequest {
    pub foundation_id: i64,

    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePaymentRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::payments::Model> for PaymentResponse {
    fn from(model: entity::payments::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            name: model.name,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl PaymentResponse {
    pub fn from_vec(models: Vec<entity::payments::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
