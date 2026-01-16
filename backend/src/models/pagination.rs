use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub total: usize,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<u64>,

    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u64>,

    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>, // "asc" or "desc"
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            search: None,
            sort_by: None,
            sort_order: Some("desc".to_string()),
        }
    }
}

impl PaginationParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(10).min(100)
    }

    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.per_page()
    }
}

impl PaginationMeta {
    pub fn new(total: usize, page: u64, per_page: u64) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u64;
        Self {
            total,
            page,
            per_page,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: usize, page: u64, per_page: u64) -> Self {
        Self {
            data,
            pagination: PaginationMeta::new(total, page, per_page),
        }
    }
}
