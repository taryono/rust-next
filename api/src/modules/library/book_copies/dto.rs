// api/src/modules/library/book_copies/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct BookCopyResponse {
    pub id: i64,
    pub book_id: i64,
    pub foundation_id: i64,
    pub inventory_number: String,
    pub barcode: Option<String>,
    pub rack_code: Option<String>,
    pub shelf_number: Option<String>,
    pub condition: String,
    pub acquisition_date: String,
    pub price: Option<i64>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BookCopyListResponse {
    pub data: Vec<BookCopyResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookCopyRequest {
    pub book_id: i64,
    pub foundation_id: i64,

    #[validate(length(min = 1, max = 100))]
    pub inventory_number: String,

    pub barcode: Option<String>,
    pub rack_code: Option<String>,
    pub shelf_number: Option<String>,
    pub condition: Option<String>,
    pub acquisition_date: Option<String>,
    pub price: Option<i64>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookCopyRequest {
    pub book_id: Option<i64>,
    pub foundation_id: Option<i64>,

    #[validate(length(min = 1, max = 100))]
    pub inventory_number: Option<String>,

    pub barcode: Option<String>,
    pub rack_code: Option<String>,
    pub shelf_number: Option<String>,
    pub condition: Option<String>,
    pub acquisition_date: Option<String>,
    pub price: Option<i64>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::book_copies::Model> for BookCopyResponse {
    fn from(model: entity::book_copies::Model) -> Self {
        Self {
            id: model.id,
            book_id: model.book_id,
            foundation_id: model.foundation_id,
            inventory_number: model.inventory_number,
            barcode: model.barcode,
            rack_code: model.rack_code,
            shelf_number: model.shelf_number,
            condition: model.condition,
            acquisition_date: model.acquisition_date.to_string(),
            price: model.price,
            status: model.status,
            notes: model.notes,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl BookCopyResponse {
    pub fn from_vec(models: Vec<entity::book_copies::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
