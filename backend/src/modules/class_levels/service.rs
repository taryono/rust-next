// ============================================================================
// backend/src/modules/class_levels/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{ClassLevelResponse, CreateClassLevelRequest, UpdateClassLevelRequest};
use super::repository::ClassLevelRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::class_levels;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct ClassLevelService {
    repository: ClassLevelRepository,
}

impl ClassLevelService {
    pub fn new(repository: ClassLevelRepository) -> Self {
        Self { repository }
    }

    /// Create new class_level with validation
    pub async fn create(
        &self,
        request: CreateClassLevelRequest,
    ) -> Result<ClassLevelResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Class Levelwith this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = class_levels::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(ClassLevelResponse::from(created))
    }

    /// Get class_level by ID
    pub async fn get_by_id(&self, id: i64) -> Result<ClassLevelResponse, AppError> {
        let class_level = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Class Levelnot found".to_string()))?;

        Ok(ClassLevelResponse::from(class_level))
    }

    /// Get all class_levels with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<ClassLevelResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<ClassLevelResponse> =
            items.into_iter().map(ClassLevelResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update class_level
    pub async fn update(
        &self,
        id: i64,
        request: UpdateClassLevelRequest,
    ) -> Result<ClassLevelResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Class Levelnot found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Class Levelwith this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = class_levels::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(ClassLevelResponse::from(updated))
    }

    /// Delete class_level
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Class Levelnot found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
