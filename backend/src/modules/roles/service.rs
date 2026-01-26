// ============================================================================
// backend/src/modules/roles/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateRoleRequest, RoleResponse, UpdateRoleRequest};
use super::repository::RoleRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::roles;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct RoleService {
    repository: RoleRepository,
}

impl RoleService {
    pub fn new(repository: RoleRepository) -> Self {
        Self { repository }
    }

    /// Create new role with validation
    pub async fn create(&self, request: CreateRoleRequest) -> Result<RoleResponse, AppError> {
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
                "Role with this name already exists".to_string(),
            ));
        }

        // Build entity with parsed dates
        let active_model = roles::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            description: Set(request.description),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(RoleResponse::from(created))
    }

    /// Get role by ID
    pub async fn get_by_id(&self, id: i64) -> Result<RoleResponse, AppError> {
        let role = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Role not found".to_string()))?;

        Ok(RoleResponse::from(role))
    }

    /// Get all roles with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<RoleResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<RoleResponse> = items.into_iter().map(RoleResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update role
    pub async fn update(
        &self,
        id: i64,
        request: UpdateRoleRequest,
    ) -> Result<RoleResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Role not found".to_string()))?;
        let name = request.name;
        // Business rule: check duplicate name if changing
        if name != existing.name {
            if let Some(_) = self
                .repository
                .find_by_name(&name, existing.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Role with this name already exists".to_string(),
                ));
            }
        }
        // Build update model
        let mut active_model = roles::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.name = Set(name);
        active_model.description = Set(request.description);

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(RoleResponse::from(updated))
    }

    /// Delete role
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Role not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related roles
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
