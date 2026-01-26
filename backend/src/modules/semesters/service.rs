// ============================================================================
// backend/src/modules/semesters/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateSemesterRequest, SemesterResponse, UpdateSemesterRequest};
use super::repository::SemesterRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::semesters;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct SemesterService {
    repository: SemesterRepository,
}

impl SemesterService {
    pub fn new(repository: SemesterRepository) -> Self {
        Self { repository }
    }

    /// Create new semester with validation
    pub async fn create(
        &self,
        request: CreateSemesterRequest,
    ) -> Result<SemesterResponse, AppError> {
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
                "Semester with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = semesters::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(SemesterResponse::from(created))
    }

    /// Get semester by ID
    pub async fn get_by_id(&self, id: i64) -> Result<SemesterResponse, AppError> {
        let semester = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Semester not found".to_string()))?;

        Ok(SemesterResponse::from(semester))
    }

    /// Get all semesters with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<SemesterResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<SemesterResponse> =
            items.into_iter().map(SemesterResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update semester
    pub async fn update(
        &self,
        id: i64,
        request: UpdateSemesterRequest,
    ) -> Result<SemesterResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Semester not found".to_string()))?;
        let name = request.name;
        // Business rule: check duplicate name if changing
        if name != existing.name {
            if let Some(_) = self
                .repository
                .find_by_name(&name, existing.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Semester with this name already exists".to_string(),
                ));
            }
        }
        // Build update model
        let mut active_model = semesters::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.name = Set(name);
        active_model.academic_calendar_id = Set(request.academic_calendar_id);
        active_model.semester_number = Set(request.semester_number);
        active_model.year = Set(request.year);
        active_model.start_date = Set(request.start_date);
        active_model.end_date = Set(request.end_date);
        active_model.is_active = Set(request.is_active as i8);

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(SemesterResponse::from(updated))
    }

    /// Delete semester
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Semester not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
