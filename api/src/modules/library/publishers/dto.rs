// api/src/modules/library/publishers/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct PublisherResponse {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PublisherListResponse {
    pub data: Vec<PublisherResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePublisherRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePublisherRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::publishers::Model> for PublisherResponse {
    fn from(model: entity::publishers::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            address: model.address,
            city: model.city,
            postal_code: model.postal_code,
            phone: model.phone,
            website: model.website,
            email: model.email,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl PublisherResponse {
    pub fn from_vec(models: Vec<entity::publishers::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
