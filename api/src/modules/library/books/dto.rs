// api/src/modules/books/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct BookResponse {
    pub id: i64,
    pub user_id: i64,
    pub foundation_id: i64,
    pub unit_id: Option<i64>,
    pub class_id: Option<i64>,
    pub name: String,
    pub student_number: String,
    pub parent_name: Option<String>,
    pub parent_phone: Option<String>,
    pub enrollment_date: Option<String>,
    pub graduation_date: Option<String>,
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
    pub unit_id: Option<i64>,
    pub class_id: Option<i64>,

    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 50))]
    pub student_number: String, // ✅ wajib ada

    pub parent_name: Option<String>,
    pub parent_phone: Option<String>,
    pub enrollment_date: Option<String>,
    pub graduation_date: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateBookRequest {
    pub unit_id: Option<i64>,
    pub class_id: Option<i64>,

    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 50))]
    pub student_number: Option<String>,

    pub parent_name: Option<String>,
    pub parent_phone: Option<String>,
    pub enrollment_date: Option<String>,
    pub graduation_date: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::students::Model> for BookResponse {
    fn from(model: entity::students::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            foundation_id: model.foundation_id,
            unit_id: model.unit_id,
            class_id: model.class_id,
            name: model.name,
            student_number: model.student_number,
            parent_name: model.parent_name,
            parent_phone: model.parent_phone,
            enrollment_date: model.enrollment_date.map(|d| d.to_rfc3339()),
            graduation_date: model.graduation_date.map(|d| d.to_rfc3339()),
            created_at: model.created_at.to_rfc3339(), // ✅ format ISO 8601
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}

impl BookResponse {
    pub fn from_vec(models: Vec<entity::students::Model>) -> Vec<Self> {
        models.into_iter().map(Self::from).collect()
    }
}
