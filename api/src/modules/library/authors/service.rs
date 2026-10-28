// ============================================================================
// api/src/modules/library/authors/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{AuthorResponse, CreateAuthorRequest, UpdateAuthorRequest};
use super::repository::AuthorRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::authors;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct AuthorService {
    repository: AuthorRepository,
}

impl AuthorService {
    pub fn new(repository: AuthorRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request: CreateAuthorRequest) -> Result<AuthorResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self.repository.find_by_full_name(&request.full_name).await? {
            return Err(AppError::ConflictError(
                "Author with this name already exists".to_string(),
            ));
        }

        let active_model = authors::ActiveModel {
            full_name: Set(request.full_name),
            biography: Set(request.biography),
            nationality: Set(request.nationality),
            email: Set(request.email),
            is_active: Set(request.is_active.unwrap_or(true)),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(AuthorResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<AuthorResponse, AppError> {
        let author = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Author not found".to_string()))?;

        Ok(AuthorResponse::from(author))
    }

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<AuthorResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<AuthorResponse> = items.into_iter().map(AuthorResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    pub async fn update(
        &self,
        id: i64,
        request: UpdateAuthorRequest,
    ) -> Result<AuthorResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Author not found".to_string()))?;

        if let Some(ref full_name) = request.full_name {
            if full_name != &existing.full_name {
                if let Some(_) = self.repository.find_by_full_name(full_name).await? {
                    return Err(AppError::ConflictError(
                        "Author with this name already exists".to_string(),
                    ));
                }
            }
        }

        let mut active_model = authors::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(full_name) = request.full_name {
            active_model.full_name = Set(full_name);
        }
        if request.biography.is_some() {
            active_model.biography = Set(request.biography);
        }
        if request.nationality.is_some() {
            active_model.nationality = Set(request.nationality);
        }
        if request.email.is_some() {
            active_model.email = Set(request.email);
        }
        if let Some(is_active) = request.is_active {
            active_model.is_active = Set(is_active);
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(AuthorResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Author not found".to_string()))?;

        self.repository.delete(id).await
    }
}
