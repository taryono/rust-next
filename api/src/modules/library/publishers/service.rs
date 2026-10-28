// ============================================================================
// api/src/modules/library/publishers/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreatePublisherRequest, PublisherResponse, UpdatePublisherRequest};
use super::repository::PublisherRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::publishers;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct PublisherService {
    repository: PublisherRepository,
}

impl PublisherService {
    pub fn new(repository: PublisherRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request: CreatePublisherRequest) -> Result<PublisherResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self.repository.find_by_name(&request.name).await? {
            return Err(AppError::ConflictError(
                "Publisher with this name already exists".to_string(),
            ));
        }

        let active_model = publishers::ActiveModel {
            name: Set(request.name),
            address: Set(request.address),
            city: Set(request.city),
            postal_code: Set(request.postal_code),
            phone: Set(request.phone),
            website: Set(request.website),
            email: Set(request.email),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(PublisherResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<PublisherResponse, AppError> {
        let publ = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Publisher not found".to_string()))?;

        Ok(PublisherResponse::from(publ))
    }

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<PublisherResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<PublisherResponse> = items.into_iter().map(PublisherResponse::from).collect();

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
        request: UpdatePublisherRequest,
    ) -> Result<PublisherResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Publisher not found".to_string()))?;

        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self.repository.find_by_name(name).await? {
                    return Err(AppError::ConflictError(
                        "Publisher with this name already exists".to_string(),
                    ));
                }
            }
        }

        let mut active_model = publishers::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }
        if request.address.is_some() {
            active_model.address = Set(request.address);
        }
        if request.city.is_some() {
            active_model.city = Set(request.city);
        }
        if request.postal_code.is_some() {
            active_model.postal_code = Set(request.postal_code);
        }
        if request.phone.is_some() {
            active_model.phone = Set(request.phone);
        }
        if request.website.is_some() {
            active_model.website = Set(request.website);
        }
        if request.email.is_some() {
            active_model.email = Set(request.email);
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(PublisherResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Publisher not found".to_string()))?;

        self.repository.delete(id).await
    }
}
