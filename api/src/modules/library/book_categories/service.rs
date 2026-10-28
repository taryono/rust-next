// ============================================================================
// api/src/modules/library/book_categories/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{BookCategoryResponse, CreateBookCategoryRequest, UpdateBookCategoryRequest};
use super::repository::BookCategoryRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::book_categories;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct BookCategoryService {
    repository: BookCategoryRepository,
}

impl BookCategoryService {
    pub fn new(repository: BookCategoryRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        request: CreateBookCategoryRequest,
    ) -> Result<BookCategoryResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Book category with this name already exists".to_string(),
            ));
        }

        let active_model = book_categories::ActiveModel {
            foundation_id: Set(request.foundation_id),
            school_id: Set(request.school_id),
            name: Set(request.name),
            code: Set(request.code),
            description: Set(request.description),
            parent_id: Set(request.parent_id),
            is_active: Set(request.is_active.unwrap_or(true)),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(BookCategoryResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<BookCategoryResponse, AppError> {
        let cat = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Book category not found".to_string()))?;

        Ok(BookCategoryResponse::from(cat))
    }

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<BookCategoryResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<BookCategoryResponse> =
            items.into_iter().map(BookCategoryResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    pub async fn update(
        &self,
        id: i64,
        request: UpdateBookCategoryRequest,
    ) -> Result<BookCategoryResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Book category not found".to_string()))?;

        let foundation_id = request.foundation_id.unwrap_or(existing.foundation_id);

        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Book category with this name already exists".to_string(),
                    ));
                }
            }
        }

        let mut active_model = book_categories::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(fid) = request.foundation_id {
            active_model.foundation_id = Set(fid);
        }
        if let Some(sid) = request.school_id {
            active_model.school_id = Set(sid);
        }
        if let Some(name) = request.name {
            active_model.name = Set(name);
        }
        if request.code.is_some() {
            active_model.code = Set(request.code);
        }
        if request.description.is_some() {
            active_model.description = Set(request.description);
        }
        if request.parent_id.is_some() {
            active_model.parent_id = Set(request.parent_id);
        }
        if let Some(is_active) = request.is_active {
            active_model.is_active = Set(is_active);
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(BookCategoryResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Book category not found".to_string()))?;

        self.repository.delete(id).await
    }
}
