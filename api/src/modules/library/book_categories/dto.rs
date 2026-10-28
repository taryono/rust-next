// api/src/modules/library/book_categories/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct BookCategoryResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub school_id: i64,
    pub name: String,
    pub code: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BookCategoryListResponse {
    pub data: Vec<BookCategoryResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookCategoryRequest {
    pub foundation_id: i64,
    pub school_id: i64,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub code: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookCategoryRequest {
    pub foundation_id: Option<i64>,
    pub school_id: Option<i64>,

    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub code: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: Option<bool>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::book_categories::Model> for BookCategoryResponse {
    fn from(model: entity::book_categories::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            school_id: model.school_id,
            name: model.name,
            code: model.code,
            description: model.description,
            parent_id: model.parent_id,
            is_active: model.is_active,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl BookCategoryResponse {
    pub fn from_vec(models: Vec<entity::book_categories::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
