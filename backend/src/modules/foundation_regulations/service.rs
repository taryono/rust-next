// ============================================================================
// backend/src/modules/foundation_regulations/service.rs
// ============================================================================
use super::dto::{
    CreateFoundationRegulationRequest, FoundationRegulationResponse,
    ToggleFoundationRegulationRequest, UpdateFoundationRegulationConfigRequest,
};
use super::repository::FoundationRegulationRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use validator::Validate;

#[derive(Clone)]
pub struct FoundationRegulationService {
    repository: FoundationRegulationRepository,
}

impl FoundationRegulationService {
    pub fn new(repository: FoundationRegulationRepository) -> Self {
        Self { repository }
    }

    // ========================================================================
    // GET SEMUA REGULASI PER YAYASAN (dengan status aktif)
    // ========================================================================

    /// Get semua regulasi + status aktif untuk yayasan tertentu
    /// Endpoint: GET /api/foundations/{foundation_code}/regulations
    pub async fn get_by_foundation_code(
        &self,
        foundation_code: String,
    ) -> Result<Vec<FoundationRegulationResponse>, AppError> {
        // Pastikan foundation exists dan tidak soft deleted
        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        // Ambil semua regulasi + LEFT JOIN foundation_regulations
        let result = self
            .repository
            .find_all_with_foundation_status(foundation.id)
            .await?;

        Ok(result)
    }

    // ========================================================================
    // CREATE (assign regulasi ke yayasan)
    // ========================================================================

    /// Assign regulasi ke yayasan
    /// Endpoint: POST /api/foundations/{foundation_code}/regulations/{regulation_code}
    pub async fn create(
        &self,
        foundation_code: String,
        regulation_code: String,
        request: CreateFoundationRegulationRequest,
    ) -> Result<FoundationRegulationResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        let regulation = self
            .repository
            .find_regulation_by_code(&regulation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        // Cek apakah sudah pernah diassign sebelumnya
        let existing = self
            .repository
            .find_by_foundation_and_regulation(foundation.id, regulation.id)
            .await?;

        if existing.is_some() {
            return Err(AppError::ConflictError(
                "Regulation already assigned to this foundation. Use toggle or update config instead."
                    .to_string(),
            ));
        }

        // Upsert (insert karena belum ada)
        let model = self
            .repository
            .upsert(
                foundation.id,
                regulation.id,
                request.is_active,
                request.config,
            )
            .await?;

        Ok(FoundationRegulationResponse {
            id: model.id,
            foundation_id: model.foundation_id,
            regulation_id: model.regulation_id,
            regulation_code: regulation.code,
            regulation_name: regulation.name,
            is_active: model.is_active.unwrap_or(0) != 0,
            config: model.config,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            deleted_at: model.deleted_at.map(|d| d.to_string()),
        })
    }

    // ========================================================================
    // GET BY ID
    // ========================================================================

    pub async fn get_by_id(&self, id: i64) -> Result<FoundationRegulationResponse, AppError> {
        let model =
            self.repository.find_by_id(id).await?.ok_or_else(|| {
                AppError::not_found("Foundation regulation not found".to_string())
            })?;

        Ok(FoundationRegulationResponse::from(model))
    }

    // ========================================================================
    // GET ALL (admin, dengan pagination)
    // ========================================================================

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<FoundationRegulationResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        Ok(PaginatedResponse::new(
            items
                .into_iter()
                .map(FoundationRegulationResponse::from)
                .collect(),
            total,
            params.page(),
            params.per_page(),
        ))
    }

    // ========================================================================
    // TOGGLE AKTIF / NONAKTIF
    // ========================================================================

    /// Toggle aktif/nonaktif regulasi untuk yayasan tertentu
    /// Endpoint: PUT /api/foundations/{foundation_code}/regulations/{regulation_code}/toggle
    pub async fn toggle(
        &self,
        foundation_code: String,
        regulation_code: String,
        request: ToggleFoundationRegulationRequest,
    ) -> Result<FoundationRegulationResponse, AppError> {
        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        let regulation = self
            .repository
            .find_regulation_by_code(&regulation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        // Upsert — config tidak diubah saat toggle
        let model = self
            .repository
            .upsert(foundation.id, regulation.id, request.is_active, None)
            .await?;

        Ok(FoundationRegulationResponse {
            id: model.id,
            foundation_id: model.foundation_id,
            regulation_id: model.regulation_id,
            regulation_code: regulation.code,
            regulation_name: regulation.name,
            is_active: model.is_active.unwrap_or(0) != 0,
            config: model.config,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            deleted_at: model.deleted_at.map(|d| d.to_string()),
        })
    }

    // ========================================================================
    // UPDATE CONFIG
    // ========================================================================

    /// Update config regulasi untuk yayasan tertentu
    /// Endpoint: PUT /api/foundations/{foundation_code}/regulations/{regulation_code}/config
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
            .find_regulation_by_code(&regulation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        // Regulasi harus sudah diassign dan aktif sebelum bisa update config
        let existing = self
            .repository
            .find_by_foundation_and_regulation(foundation.id, regulation.id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(
                    "Regulation not assigned to this foundation yet. Create it first.".to_string(),
                )
            })?;

        if existing.is_active.unwrap_or(0) == 0 {
            return Err(AppError::validation(
                "Cannot update config for inactive regulation. Toggle it active first.".to_string(),
            ));
        }

        let model = self
            .repository
            .upsert(
                foundation.id,
                regulation.id,
                true, // tetap aktif
                Some(request.config),
            )
            .await?;

        Ok(FoundationRegulationResponse {
            id: model.id,
            foundation_id: model.foundation_id,
            regulation_id: model.regulation_id,
            regulation_code: regulation.code,
            regulation_name: regulation.name,
            is_active: model.is_active.unwrap_or(0) != 0,
            config: model.config,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            deleted_at: model.deleted_at.map(|d| d.to_string()),
        })
    }

    // ========================================================================
    // SOFT DELETE
    // ========================================================================

    /// Soft delete foundation regulation
    /// Endpoint: DELETE /api/foundations/{foundation_code}/regulations/{regulation_code}
    pub async fn delete(
        &self,
        foundation_code: String,
        regulation_code: String,
    ) -> Result<(), AppError> {
        let foundation = self
            .repository
            .find_foundation_by_code(&foundation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation not found".to_string()))?;

        let regulation = self
            .repository
            .find_regulation_by_code(&regulation_code)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        let existing = self
            .repository
            .find_by_foundation_and_regulation(foundation.id, regulation.id)
            .await?
            .ok_or_else(|| AppError::not_found("Foundation regulation not found".to_string()))?;

        self.repository.soft_delete(existing.id).await
    }
}
