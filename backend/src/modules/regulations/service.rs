// ============================================================================
// backend/src/modules/regulations/service.rs
// ============================================================================
use super::dto::{
    CreateRegulationRequest, FoundationRegulationResponse, RegulationResponse,
    ToggleRegulationRequest, UpdateFoundationRegulationConfigRequest, UpdateRegulationRequest,
};
use super::repository::RegulationRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::regulations;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct RegulationService {
    repository: RegulationRepository,
}

impl RegulationService {
    pub fn new(repository: RegulationRepository) -> Self {
        Self { repository }
    }

    // ========================================================================
    // MASTER REGULATION
    // ========================================================================

    pub async fn create(
        &self,
        request: CreateRegulationRequest,
    ) -> Result<RegulationResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if self.repository.find_by_code(&request.code).await?.is_some() {
            return Err(AppError::ConflictError(
                "Regulation with this code already exists".to_string(),
            ));
        }

        let active_model = regulations::ActiveModel {
            code: Set(request.code),
            name: Set(request.name),
            description: Set(request.description),
            config_schema: Set(request.config_schema),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(RegulationResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<RegulationResponse, AppError> {
        let regulation = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        Ok(RegulationResponse::from(regulation))
    }

    // ✅ Hapus ctx: &ServiceContext
    pub async fn get_all(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<RegulationResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params).await?;

        Ok(PaginatedResponse::new(
            items.into_iter().map(RegulationResponse::from).collect(),
            total,
            params.page(),
            params.per_page(),
        ))
    }

    pub async fn update(
        &self,
        id: i64,
        request: UpdateRegulationRequest,
    ) -> Result<RegulationResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        let mut active_model = regulations::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }
        if let Some(description) = request.description {
            active_model.description = Set(Some(description));
        }
        if let Some(config_schema) = request.config_schema {
            active_model.config_schema = Set(Some(config_schema));
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(RegulationResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        let in_use = self.repository.is_regulation_in_use(id).await?;
        if in_use {
            return Err(AppError::ConflictError(
                "Cannot delete regulation that is still used by foundations".to_string(),
            ));
        }

        self.repository.delete(id).await
    }

    // ========================================================================
    // FOUNDATION REGULATION
    // ========================================================================

    pub async fn get_by_foundation_code(
        &self,
        foundation_code: String,
    ) -> Result<Vec<FoundationRegulationResponse>, AppError> {
        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        let items = self
            .repository
            .find_all_with_foundation_status(foundation.id)
            .await?;

        Ok(items)
    }

    pub async fn toggle(
        &self,
        foundation_code: String,
        regulation_code: String,
        request: ToggleRegulationRequest,
    ) -> Result<FoundationRegulationResponse, AppError> {
        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        let regulation = self
            .repository
            .find_by_code(&regulation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        // ✅ bool → Option<i8>
        let result = self
            .repository
            .upsert_foundation_regulation(
                foundation.id,
                regulation.id,
                Some(request.is_active as i8),
                None,
            )
            .await?;

        Ok(result)
    }

    pub async fn update_config(
        &self,
        foundation_code: String,
        regulation_code: String,
        request: UpdateFoundationRegulationConfigRequest,
    ) -> Result<FoundationRegulationResponse, AppError> {
        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        let regulation = self
            .repository
            .find_by_code(&regulation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        let existing = self
            .repository
            .find_foundation_regulation(foundation.id, regulation.id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(
                    "Regulation is not assigned to this foundation yet. Toggle it first."
                        .to_string(),
                )
            })?;

        // ✅ Option<i8> → cek dengan unwrap_or
        if existing.is_active.unwrap_or(0) == 0 {
            return Err(AppError::validation(
                "Cannot update config for inactive regulation. Activate it first.".to_string(),
            ));
        }

        let result = self
            .repository
            .upsert_foundation_regulation(
                foundation.id,
                regulation.id,
                Some(existing.is_active.unwrap_or(0)), // ✅ sudah Option<i8>
                Some(request.config),
            )
            .await?;

        Ok(result)
    }
}
