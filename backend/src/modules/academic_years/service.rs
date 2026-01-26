// ============================================================================
// backend/src/modules/academic_years/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{AcademicYearResponse, CreateAcademicYearRequest, UpdateAcademicYearRequest};
use super::repository::AcademicYearRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use chrono::NaiveDate;
use entity::academic_years;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct AcademicYearService {
    repository: AcademicYearRepository,
}

impl AcademicYearService {
    pub fn new(repository: AcademicYearRepository) -> Self {
        Self { repository }
    }

    /// Create new academic year with validation
    pub async fn create(
        &self,
        request: CreateAcademicYearRequest,
    ) -> Result<AcademicYearResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Parse dates
        let start_date =
            NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d").map_err(|_| {
                AppError::ValidationError("Invalid start_date format. Use YYYY-MM-DD".to_string())
            })?;

        let end_date = NaiveDate::parse_from_str(&request.end_date, "%Y-%m-%d").map_err(|_| {
            AppError::ValidationError("Invalid end_date format. Use YYYY-MM-DD".to_string())
        })?;

        // Business rule: end_date must be after start_date
        if end_date <= start_date {
            return Err(AppError::ValidationError(
                "End date must be after start date".to_string(),
            ));
        }

        // Check overlap (pakai string comparison bisa juga untuk date)
        let has_overlap = self
            .repository
            .check_date_overlap(
                &request.start_date,
                &request.end_date,
                request.foundation_id,
                None,
            )
            .await?;

        if has_overlap {
            return Err(AppError::ConflictError(
                "Academic year dates overlap with existing academic year".to_string(),
            ));
        }

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Academic year with this name already exists".to_string(),
            ));
        }

        // If setting as active, deactivate others first
        let is_active = request.is_active.unwrap_or(0);
        if is_active == 1 {
            self.repository
                .deactivate_all(request.foundation_id)
                .await?;
        }

        // Build entity with parsed dates
        let active_model = academic_years::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            start_date: Set(start_date), // ← NaiveDate, bukan String
            end_date: Set(end_date),     // ← NaiveDate, bukan String
            is_active: Set(is_active),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(AcademicYearResponse::from(created))
    }

    /// Get academic year by ID
    pub async fn get_by_id(&self, id: i64) -> Result<AcademicYearResponse, AppError> {
        let academic_year = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Academic year not found".to_string()))?;

        Ok(AcademicYearResponse::from(academic_year))
    }

    /// Get all academic years with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<AcademicYearResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<AcademicYearResponse> =
            items.into_iter().map(AcademicYearResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Get active academic year for a foundation
    pub async fn get_active(
        &self,
        foundation_id: i64,
    ) -> Result<Option<AcademicYearResponse>, AppError> {
        let academic_year = self.repository.find_active(foundation_id).await?;
        Ok(academic_year.map(AcademicYearResponse::from))
    }

    /// Update academic year
    pub async fn update(
        &self,
        id: i64,
        request: UpdateAcademicYearRequest,
    ) -> Result<AcademicYearResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Academic year not found".to_string()))?;
        // Parse dates if provided
        let start_date = match request.start_date.as_ref() {
            Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| AppError::validation("Invalid start_date format"))?,
            None => existing.start_date,
        };

        let end_date = match request.end_date.as_ref() {
            Some(date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|_| AppError::validation("Invalid end_date format"))?,
            None => existing.end_date,
        };

        if request.start_date.is_some() || request.end_date.is_some() {
            let has_overlap = self
                .repository
                .check_date_overlap(
                    &start_date.to_string(),
                    &end_date.to_string(),
                    existing.foundation_id,
                    Some(id),
                )
                .await?;

            if has_overlap {
                return Err(AppError::ConflictError(
                    "Academic year dates overlap with existing academic year".to_string(),
                ));
            }
        }

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Academic year with this name already exists".to_string(),
                    ));
                }
            }
        }

        // Business rule: if setting as active, deactivate others
        if request.is_active == Some(1) {
            self.repository
                .deactivate_all(existing.foundation_id)
                .await?;
        }

        // Build update model
        let mut active_model = academic_years::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }
        if let Some(start_date) = request.start_date.clone() {
            active_model.start_date = Set(chrono::NaiveDate::parse_from_str(
                &start_date,
                "%Y-%m-%d",
            )
            .map_err(|_| {
                AppError::ValidationError("Invalid start_date format. Use YYYY-MM-DD".to_string())
            })?);
        }
        if let Some(end_date) = request.end_date.clone() {
            active_model.end_date = Set(chrono::NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
                .map_err(|_| {
                AppError::ValidationError("Invalid end_date format. Use YYYY-MM-DD".to_string())
            })?);
        }
        if let Some(is_active) = request.is_active {
            active_model.is_active = Set(is_active);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(AcademicYearResponse::from(updated))
    }

    /// Delete academic year
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Academic year not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
