// ============================================================================
// api/src/modules/payments/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreatePaymentRequest, PaymentResponse, UpdatePaymentRequest};
use super::repository::PaymentRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::payments;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct PaymentService {
    repository: PaymentRepository,
}

impl PaymentService {
    pub fn new(repository: PaymentRepository) -> Self {
        Self { repository }
    }

    /// Create new student with validation
    pub async fn create(&self, request: CreatePaymentRequest) -> Result<PaymentResponse, AppError> {
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
                "Payment with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = payments::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(PaymentResponse::from(created))
    }

    /// Get student by ID
    pub async fn get_by_id(&self, id: i64) -> Result<PaymentResponse, AppError> {
        let student = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Payment not found".to_string()))?;

        Ok(PaymentResponse::from(student))
    }

    /// Get all payments with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<PaymentResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<PaymentResponse> =
            items.into_iter().map(PaymentResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update student
    pub async fn update(
        &self,
        id: i64,
        request: UpdatePaymentRequest,
    ) -> Result<PaymentResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Payment not found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Payment with this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = payments::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(PaymentResponse::from(updated))
    }

    /// Delete student
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Payment not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related payments
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
