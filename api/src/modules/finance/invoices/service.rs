// ============================================================================
// api/src/modules/finance/invoices/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateInvoiceRequest, InvoiceResponse, UpdateInvoiceRequest};
use super::repository::InvoiceRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::invoices;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct InvoiceService {
    repository: InvoiceRepository,
}

impl InvoiceService {
    pub fn new(repository: InvoiceRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request: CreateInvoiceRequest) -> Result<InvoiceResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self.repository.find_by_code(&request.code).await? {
            return Err(AppError::ConflictError(
                "Invoice with this code already exists".to_string(),
            ));
        }

        let active_model = invoices::ActiveModel {
            code: Set(request.code),
            name: Set(request.name),
            description: Set(request.description),
            config_schema: Set(request.config_schema),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(InvoiceResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<InvoiceResponse, AppError> {
        let invoice = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Invoice not found".to_string()))?;

        Ok(InvoiceResponse::from(invoice))
    }

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<InvoiceResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<InvoiceResponse> =
            items.into_iter().map(InvoiceResponse::from).collect();

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
        request: UpdateInvoiceRequest,
    ) -> Result<InvoiceResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Invoice not found".to_string()))?;

        if let Some(ref code) = request.code {
            if code != &existing.code {
                if let Some(_) = self.repository.find_by_code(code).await? {
                    return Err(AppError::ConflictError(
                        "Invoice with this code already exists".to_string(),
                    ));
                }
            }
        }

        let mut active_model = invoices::ActiveModel {
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
        if request.description.is_some() {
            active_model.description = Set(request.description);
        }
        if request.config_schema.is_some() {
            active_model.config_schema = Set(request.config_schema);
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(InvoiceResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Invoice not found".to_string()))?;

        self.repository.delete(id).await
    }
}
