// ============================================================================
// api/src/modules/library/book_copies/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{BookCopyResponse, CreateBookCopyRequest, UpdateBookCopyRequest};
use super::repository::BookCopyRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use chrono::NaiveDate;
use entity::book_copies;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct BookCopyService {
    repository: BookCopyRepository,
}

impl BookCopyService {
    pub fn new(repository: BookCopyRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        request: CreateBookCopyRequest,
    ) -> Result<BookCopyResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self
            .repository
            .find_by_inventory_number(&request.inventory_number, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "BookCopy with this inventory number already exists".to_string(),
            ));
        }

        let acquisition_date = request
            .acquisition_date
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Utc::now().date_naive());

        let active_model = book_copies::ActiveModel {
            book_id: Set(request.book_id),
            foundation_id: Set(request.foundation_id),
            inventory_number: Set(request.inventory_number),
            barcode: Set(request.barcode),
            rack_code: Set(request.rack_code),
            shelf_number: Set(request.shelf_number),
            condition: Set(request.condition.unwrap_or_else(|| "good".to_string())),
            acquisition_date: Set(acquisition_date),
            price: Set(request.price),
            status: Set(request.status.unwrap_or_else(|| "available".to_string())),
            notes: Set(request.notes),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(BookCopyResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<BookCopyResponse, AppError> {
        let copy = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("BookCopy not found".to_string()))?;

        Ok(BookCopyResponse::from(copy))
    }

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<BookCopyResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<BookCopyResponse> =
            items.into_iter().map(BookCopyResponse::from).collect();

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
        request: UpdateBookCopyRequest,
    ) -> Result<BookCopyResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("BookCopy not found".to_string()))?;

        let foundation_id = request.foundation_id.unwrap_or(existing.foundation_id);

        if let Some(ref inv) = request.inventory_number {
            if inv != &existing.inventory_number {
                if let Some(_) = self
                    .repository
                    .find_by_inventory_number(inv, foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "BookCopy with this inventory number already exists".to_string(),
                    ));
                }
            }
        }

        let mut active_model = book_copies::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(bid) = request.book_id {
            active_model.book_id = Set(bid);
        }
        if let Some(fid) = request.foundation_id {
            active_model.foundation_id = Set(fid);
        }
        if let Some(inv) = request.inventory_number {
            active_model.inventory_number = Set(inv);
        }
        if request.barcode.is_some() {
            active_model.barcode = Set(request.barcode);
        }
        if request.rack_code.is_some() {
            active_model.rack_code = Set(request.rack_code);
        }
        if request.shelf_number.is_some() {
            active_model.shelf_number = Set(request.shelf_number);
        }
        if let Some(cond) = request.condition {
            active_model.condition = Set(cond);
        }
        if let Some(d) = request.acquisition_date {
            if let Ok(parsed) = NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                active_model.acquisition_date = Set(parsed);
            }
        }
        if request.price.is_some() {
            active_model.price = Set(request.price);
        }
        if let Some(status) = request.status {
            active_model.status = Set(status);
        }
        if request.notes.is_some() {
            active_model.notes = Set(request.notes);
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(BookCopyResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("BookCopy not found".to_string()))?;

        self.repository.delete(id).await
    }
}
