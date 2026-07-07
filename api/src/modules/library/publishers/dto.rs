// api/src/modules/publisher/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct BookResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub title: String,
    pub nib: String,
    pub author: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BookListResponse {
    pub data: Vec<BookResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookRequest {
    pub user_id: i64,
    pub foundation_id: i64,

    #[validate(length(min = 3, max = 100))]
    pub title: String,

    #[validate(length(min = 1, max = 50))]
    pub nib: String,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookRequest {
    pub foundation_id: Option<i64>,
    #[validate(length(min = 1, max = 50))]
    pub title: Option<String>,
    pub nib: Option<String>,
    pub author: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::publisher::Model> for BookResponse {
    fn from(model: entity::publisher::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            title: model.title,
            nib: model.nib,
            author: Some(model.author).unwrap_or_default(),
            created_at: model.created_at.to_rfc3339(), // ✅ format ISO 8601
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl BookResponse {
    pub fn from_vec(models: Vec<entity::publisher::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
