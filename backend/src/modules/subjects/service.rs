// ============================================================================
// backend/src/modules/subjects/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateSubjectRequest, SubjectResponse, UpdateSubjectRequest};
use super::repository::SubjectRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::subjects;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct SubjectService {
    repository: SubjectRepository,
}

impl SubjectService {
    pub fn new(repository: SubjectRepository) -> Self {
        Self { repository }
    }

    /// Create new subject with validation
    pub async fn create(&self, request: CreateSubjectRequest) -> Result<SubjectResponse, AppError> {
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
                "Subject with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = subjects::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(SubjectResponse::from(created))
    }

    /// Get subject by ID
    pub async fn get_by_id(&self, id: i64) -> Result<SubjectResponse, AppError> {
        let subject = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Subject not found".to_string()))?;

        Ok(SubjectResponse::from(subject))
    }

    /// Get all subjects with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<SubjectResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<SubjectResponse> =
            items.into_iter().map(SubjectResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update subject
    pub async fn update(
        &self,
        id: i64,
        request: UpdateSubjectRequest,
    ) -> Result<SubjectResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Subject not found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Subject with this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = subjects::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(SubjectResponse::from(updated))
    }

    /// Delete subject
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Subject not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
