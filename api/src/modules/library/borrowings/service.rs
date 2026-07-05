// ============================================================================
// api/src/modules/borrowings/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateBorrowingRequest, BorrowingResponse, UpdateBorrowingRequest};
use super::repository::BorrowingRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::students;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct BorrowingService {
    repository: BorrowingRepository,
}

impl BorrowingService {
    pub fn new(repository: BorrowingRepository) -> Self {
        Self { repository }
    }

    /// Create new student with validation
    pub async fn create(&self, request: CreateBorrowingRequest) -> Result<BorrowingResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Borrowing with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = students::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(BorrowingResponse::from(created))
    }

    /// Get student by ID
    pub async fn get_by_id(&self, id: i64) -> Result<BorrowingResponse, AppError> {
        let student = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Borrowing not found".to_string()))?;

        Ok(BorrowingResponse::from(student))
    }

    /// Get all students with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<BorrowingResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<BorrowingResponse> =
            items.into_iter().map(BorrowingResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update student
    pub async fn update(
        &self,
        id: i64,
        request: UpdateBorrowingRequest,
    ) -> Result<BorrowingResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Borrowing not found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Borrowing with this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = students::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(BorrowingResponse::from(updated))
    }

    /// Delete student
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Borrowing not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related students
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
