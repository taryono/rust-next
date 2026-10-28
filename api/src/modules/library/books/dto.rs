// api/src/modules/library/books/dto.rs
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
    pub school_id: i64,
    pub isbn: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub edition: Option<String>,
    pub language: String,
    pub author_id: Option<i64>,
    pub publisher_id: Option<i64>,
    pub publication_year: Option<i16>,
    pub book_category_id: Option<i64>,
    pub classification_code: Option<String>,
    pub pages: Option<i32>,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub rack_location: Option<String>,
    pub total_stock: i32,
    pub available_stock: i32,
    pub price: Option<i64>,
    pub source: Option<String>,
    pub status: String,
    pub notes: Option<String>,
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
    pub foundation_id: i64,
    pub school_id: i64,

    #[validate(length(min = 1, max = 255))]
    pub title: String,

    pub isbn: Option<String>,
    pub subtitle: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub author_id: Option<i64>,
    pub publisher_id: Option<i64>,
    pub publication_year: Option<i16>,
    pub book_category_id: Option<i64>,
    pub classification_code: Option<String>,
    pub pages: Option<i32>,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub rack_location: Option<String>,
    pub total_stock: Option<i32>,
    pub available_stock: Option<i32>,
    pub price: Option<i64>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookRequest {
    pub foundation_id: Option<i64>,
    pub school_id: Option<i64>,

    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,

    pub isbn: Option<String>,
    pub subtitle: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub author_id: Option<i64>,
    pub publisher_id: Option<i64>,
    pub publication_year: Option<i16>,
    pub book_category_id: Option<i64>,
    pub classification_code: Option<String>,
    pub pages: Option<i32>,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub rack_location: Option<String>,
    pub total_stock: Option<i32>,
    pub available_stock: Option<i32>,
    pub price: Option<i64>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::books::Model> for BookResponse {
    fn from(model: entity::books::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            school_id: model.school_id,
            isbn: model.isbn,
            title: model.title,
            subtitle: model.subtitle,
            edition: model.edition,
            language: model.language,
            author_id: model.author_id,
            publisher_id: model.publisher_id,
            publication_year: model.publication_year,
            book_category_id: model.book_category_id,
            classification_code: model.classification_code,
            pages: model.pages,
            synopsis: model.synopsis,
            cover_url: model.cover_url,
            rack_location: model.rack_location,
            total_stock: model.total_stock,
            available_stock: model.available_stock,
            price: model.price,
            source: model.source,
            status: model.status,
            notes: model.notes,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl BookResponse {
    pub fn from_vec(models: Vec<entity::books::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
