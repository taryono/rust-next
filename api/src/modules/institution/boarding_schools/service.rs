// ============================================================================
// api/src/modules/institution/boarding_school/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{
    BoardingSchoolResponse, CreateBoardingSchoolRequest, UpdateBoardingSchoolRequest,
};
use super::repository::BoardingSchoolRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::boarding_schools;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct BoardingSchoolService {
    repository: BoardingSchoolRepository,
}

impl BoardingSchoolService {
    pub fn new(repository: BoardingSchoolRepository) -> Self {
        Self { repository }
    }

    /// Create new class with validation
    pub async fn create(
        &self,
        request: CreateBoardingSchoolRequest,
    ) -> Result<BoardingSchoolResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self.repository.find_by_name(&request.name).await? {
            return Err(AppError::ConflictError(
                "BoardingSchool with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = boarding_schools::ActiveModel {
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(BoardingSchoolResponse::from(created))
    }

    /// Get class by ID
    pub async fn get_by_id(&self, id: i64) -> Result<BoardingSchoolResponse, AppError> {
        let boarding_schools = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("BoardingSchool not found".to_string()))?;

        Ok(BoardingSchoolResponse::from(boarding_schools))
    }

    /// Get all boarding_schools with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<BoardingSchoolResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params).await?;

        let responses: Vec<BoardingSchoolResponse> = items
            .into_iter()
            .map(BoardingSchoolResponse::from)
            .collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update class
    pub async fn update(
        &self,
        id: i64,
        request: UpdateBoardingSchoolRequest,
    ) -> Result<BoardingSchoolResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("BoardingSchool not found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self.repository.find_by_name(name).await? {
                    return Err(AppError::ConflictError(
                        "BoardingSchool with this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = boarding_schools::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(BoardingSchoolResponse::from(updated))
    }

    /// Delete class
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("BoardingSchool not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
