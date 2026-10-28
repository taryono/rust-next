// ============================================================================
// api/src/modules/admission/registrations/service.rs
// ============================================================================
use super::dto::{CreateRegistrationRequest, RegistrationResponse, UpdateRegistrationRequest};
use super::repository::RegistrationRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};

use entity::registrations;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct RegistrationService {
    repository: RegistrationRepository,
}

impl RegistrationService {
    pub fn new(repository: RegistrationRepository) -> Self {
        Self { repository }
    }

    /// Create new registration with validation
    pub async fn create(
        &self,
        request: CreateRegistrationRequest,
    ) -> Result<RegistrationResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Registration with this name already exists".to_string(),
            ));
        }

        let active_model = registrations::ActiveModel {
            foundation_id: Set(request.foundation_id),
            code: Set(request.code),
            name: Set(request.name),
            address: Set(request.address),
            config_schema: Set(request.config_schema),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(RegistrationResponse::from(created))
    }

    /// Get registration by ID
    pub async fn get_by_id(&self, id: i64) -> Result<RegistrationResponse, AppError> {
        let registration = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Registration not found".to_string()))?;

        Ok(RegistrationResponse::from(registration))
    }

    /// Get all registrations with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<RegistrationResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self
            .repository
            .find_all(&params, foundation_id.unwrap_or(1))
            .await?;

        let responses: Vec<RegistrationResponse> =
            items.into_iter().map(RegistrationResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update registration
    pub async fn update(
        &self,
        id: i64,
        request: UpdateRegistrationRequest,
    ) -> Result<RegistrationResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Registration not found".to_string()))?;

        let mut active_model = registrations::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(code) = request.code {
            active_model.code = Set(code);
        }
        if let Some(name) = request.name {
            active_model.name = Set(name);
        }
        if let Some(address) = request.address {
            active_model.address = Set(Some(address));
        }
        if let Some(config_schema) = request.config_schema {
            active_model.config_schema = Set(Some(config_schema));
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(RegistrationResponse::from(updated))
    }

    /// Delete registration
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Registration not found".to_string()))?;

        self.repository.delete(id).await
    }
}
