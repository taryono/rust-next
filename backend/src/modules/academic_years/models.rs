use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademicYearResponse {
    pub id: u64,
    pub foundation_id: u64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_active: Option<i8>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AcademicYearListResponse {
    pub data: Vec<AcademicYearResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAcademicYearRequest {
    pub foundation_id: u64,

    #[validate(length(min = 3))]
    pub name: String,

    pub start_date: String,
    pub end_date: String,

    pub is_active: Option<i8>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAcademicYearRequest {
    #[validate(length(min = 3))]
    pub name: Option<String>,

    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_active: Option<i8>,
}
