// ============================================================================
// backend/src/modules/attendances/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{AttendanceResponse, CreateAttendanceRequest, UpdateAttendanceRequest};
use super::repository::AttendanceRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::attendances;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct AttendanceService {
    repository: AttendanceRepository,
}

impl AttendanceService {
    pub fn new(repository: AttendanceRepository) -> Self {
        Self { repository }
    }

    /// Create new attendance with validation
    pub async fn create(
        &self,
        request: CreateAttendanceRequest,
    ) -> Result<AttendanceResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_date(&request.date.to_string(), request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Attendance with this date already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = attendances::ActiveModel {
            foundation_id: Set(request.foundation_id),
            student_id: Set(request.student_id),
            class_subject_id: Set(request.class_subject_id),
            date: Set(request.date),
            status: Set(request.status),
            notes: Set(request.notes),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(AttendanceResponse::from(created))
    }

    /// Get attendance by ID
    pub async fn get_by_id(&self, id: i64) -> Result<AttendanceResponse, AppError> {
        let attendance = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Attendance not found".to_string()))?;

        Ok(AttendanceResponse::from(attendance))
    }

    /// Get all attendances with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: i64,
    ) -> Result<PaginatedResponse<AttendanceResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<AttendanceResponse> =
            items.into_iter().map(AttendanceResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update attendance
    pub async fn update(
        &self,
        id: i64,
        request: UpdateAttendanceRequest,
    ) -> Result<AttendanceResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Attendance not found".to_string()))?;

        // Langsung gunakan let biasa
        let date = request.date;
        if date != existing.date {
            if let Some(_) = self
                .repository
                .find_by_date(&date.to_string(), request.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Attendance with this date already exists".to_string(),
                ));
            }
        }
        // Build update model
        let mut active_model = attendances::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Langsung gunakan let biasa
        let date = request.date;
        if date != existing.date {
            if let Some(_) = self
                .repository
                .find_by_date(&date.to_string(), request.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Attendance with this date already exists".to_string(),
                ));
            }
        }
        active_model.status = Set(request.status);
        active_model.student_id = Set(request.student_id);
        active_model.date = Set(request.date);
        active_model.notes = Set(request.notes.clone());
        active_model.class_subject_id = Set(request.class_subject_id);
        active_model.foundation_id = Set(request.foundation_id);

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(AttendanceResponse::from(updated))
    }

    /// Delete attendance
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Attendance not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
