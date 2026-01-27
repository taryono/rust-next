// ============================================================================
// backend/src/modules/settings/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateSettingRequest, SettingResponse, UpdateSettingRequest};
use super::repository::SettingRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::settings;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct SettingService {
    repository: SettingRepository,
}

impl SettingService {
    pub fn new(repository: SettingRepository) -> Self {
        Self { repository }
    }

    /// Create new setting with validation
    pub async fn create(&self, request: CreateSettingRequest) -> Result<SettingResponse, AppError> {
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
                "Setting with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = settings::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(SettingResponse::from(created))
    }

    /// Get setting by ID
    pub async fn get_by_id(&self, id: i64) -> Result<SettingResponse, AppError> {
        let setting = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Setting not found".to_string()))?;

        Ok(SettingResponse::from(setting))
    }

    /// Get all settings with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<SettingResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<SettingResponse> =
            items.into_iter().map(SettingResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update setting
    pub async fn update(
        &self,
        id: i64,
        request: UpdateSettingRequest,
    ) -> Result<SettingResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Setting not found".to_string()))?;
        let name = request.name;
        // Business rule: check duplicate name if changing
        if name != existing.name {
            if let Some(_) = self
                .repository
                .find_by_name(&name, existing.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Setting with this name already exists".to_string(),
                ));
            }
        }
        // Build update model
        let mut active_model = settings::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.name = Set(name);
        active_model.academic_year_id = Set(request.academic_year_id);
        active_model.start_date = Set(request.start_date);
        active_model.end_date = Set(request.end_date);
        active_model.is_active = Set(request.is_active as i8);

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(SettingResponse::from(updated))
    }

    /// Delete setting
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Setting not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related settings
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
