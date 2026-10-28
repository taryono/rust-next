// api/src/modules/library/authors/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthorResponse {
    pub id: i64,
    pub full_name: String,
    pub biography: Option<String>,
    pub nationality: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthorListResponse {
    pub data: Vec<AuthorResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAuthorRequest {
    #[validate(length(min = 1, max = 255))]
    pub full_name: String,
    pub biography: Option<String>,
    pub nationality: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAuthorRequest {
    #[validate(length(min = 1, max = 255))]
    pub full_name: Option<String>,
    pub biography: Option<String>,
    pub nationality: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::authors::Model> for AuthorResponse {
    fn from(model: entity::authors::Model) -> Self {
        Self {
            id: model.id,
            full_name: model.full_name,
            biography: model.biography,
            nationality: model.nationality,
            email: model.email,
            is_active: model.is_active,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl AuthorResponse {
    pub fn from_vec(models: Vec<entity::authors::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
