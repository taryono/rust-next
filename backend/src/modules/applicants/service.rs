// ============================================================================
// backend/src/modules/applicants/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{ApplicantResponse, CreateApplicantRequest, UpdateApplicantRequest};
use super::repository::ApplicantRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};

use entity::applicants;
use sea_orm::Set;
use validator::Validate;
#[derive(Clone)]
pub struct ApplicantService {
    repository: ApplicantRepository,
}

impl ApplicantService {
    pub fn new(repository: ApplicantRepository) -> Self {
        Self { repository }
    }

    /// Create new applicant with validation
    pub async fn create(
        &self,
        request: CreateApplicantRequest,
    ) -> Result<ApplicantResponse, AppError> {
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
                "Applicant with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = applicants::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            birth_place: Set(request.birth_place),
            birth_date: Set(request.birth_date),
            gender: Set(request.gender),
            email: Set(request.email),
            phone: Set(request.phone),
            address: Set(request.address),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(ApplicantResponse::from(created))
    }

    /// Get applicant by ID
    pub async fn get_by_id(&self, id: i64) -> Result<ApplicantResponse, AppError> {
        let applicant = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Applicant not found".to_string()))?;

        Ok(ApplicantResponse::from(applicant))
    }

    /// Get all applicants with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<ApplicantResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self
            .repository
            .find_all(&params, foundation_id.unwrap())
            .await?;

        let responses: Vec<ApplicantResponse> =
            items.into_iter().map(ApplicantResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update applicant
    pub async fn update(
        &self,
        id: i64,
        request: UpdateApplicantRequest,
    ) -> Result<ApplicantResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Applicant not found".to_string()))?;

        // Build update model
        let mut active_model = applicants::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.name = Set(request.name);
        active_model.birth_place = Set(request.birth_place);
        active_model.birth_date = Set(request.birth_date);
        active_model.gender = Set(request.gender);
        active_model.email = Set(request.email);
        active_model.phone = Set(request.phone);
        active_model.address = Set(request.address);
        active_model.updated_at = Set(chrono::Utc::now());

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(ApplicantResponse::from(updated))
    }

    /// Delete applicant
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Applicant not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
