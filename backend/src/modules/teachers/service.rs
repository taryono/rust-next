// ============================================================================
// backend/src/modules/teachers/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateTeacherRequest, TeacherResponse, UpdateTeacherRequest};
use super::repository::TeacherRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::teachers;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct TeacherService {
    repository: TeacherRepository,
}

impl TeacherService {
    pub fn new(repository: TeacherRepository) -> Self {
        Self { repository }
    }

    /// Create new teacher with validation
    pub async fn create(&self, request: CreateTeacherRequest) -> Result<TeacherResponse, AppError> {
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
                "Teacher with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let mut active_model = teachers::ActiveModel {
            foundation_id: Set(request.foundation_id),
            user_id: Set(request.user_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.unit_id = Set(request.unit_id);
        active_model.nik = Set(request.nik);
        active_model.employee_number = Set(request.employee_number);
        active_model.specialization = Set(request.specialization);
        active_model.qualification = Set(request.qualification);
        active_model.salary = Set(request.salary);
        active_model.employment_status = Set(request.employment_status);
        active_model.end_date = Set(request.end_date); // ← Tambah ini (good practice)
        active_model.hire_date = Set(request.hire_date);
        active_model.created_at = Set(chrono::Utc::now());
        active_model.updated_at = Set(chrono::Utc::now());
        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(TeacherResponse::from(created))
    }

    /// Get teacher by ID
    pub async fn get_by_id(&self, id: i64) -> Result<TeacherResponse, AppError> {
        let teacher = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Teacher not found".to_string()))?;

        Ok(TeacherResponse::from(teacher))
    }

    /// Get all teachers with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<TeacherResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<TeacherResponse> =
            items.into_iter().map(TeacherResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update teacher
    pub async fn update(
        &self,
        id: i64,
        request: UpdateTeacherRequest,
    ) -> Result<TeacherResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Teacher not found".to_string()))?;

        // Business rule: check duplicate name if changing
        let name = request.name.clone();
        if name != existing.name {
            if let Some(_) = self
                .repository
                .find_by_name(&name, existing.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Teacher with this name already exists".to_string(),
                ));
            }
        }
        // Build update model
        let mut active_model = teachers::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.unit_id = Set(request.unit_id);
        active_model.nik = Set(request.nik);
        active_model.employee_number = Set(request.employee_number);
        active_model.specialization = Set(request.specialization);
        active_model.qualification = Set(request.qualification);
        active_model.salary = Set(request.salary);
        active_model.employment_status = Set(request.employment_status);
        active_model.end_date = Set(request.end_date); // ← Tambah ini (good practice)
        active_model.hire_date = Set(request.hire_date);
        active_model.created_at = Set(chrono::Utc::now());
        active_model.updated_at = Set(chrono::Utc::now());

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(TeacherResponse::from(updated))
    }

    /// Delete teacher
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Teacher not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related teachers
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
