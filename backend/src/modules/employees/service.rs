// ============================================================================
// backend/src/modules/employees/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateEmployeeRequest, EmployeeResponse, UpdateEmployeeRequest};
use super::repository::EmployeeRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::employees;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct EmployeeService {
    repository: EmployeeRepository,
}

impl EmployeeService {
    pub fn new(repository: EmployeeRepository) -> Self {
        Self { repository }
    }

    /// Create new employee with validation
    pub async fn create(
        &self,
        request: CreateEmployeeRequest,
    ) -> Result<EmployeeResponse, AppError> {
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
                "Employee with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = employees::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            unit_id: Set(request.unit_id),
            nik: Set(request.nik),
            employee_number: Set(request.employee_number),
            specialization: Set(request.specialization),
            qualification: Set(request.qualification),
            hire_date: Set(request.hire_date),
            end_date: Set(request.end_date),
            salary: Set(request.salary),
            employment_type: Set(request.employment_type),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(EmployeeResponse::from(created))
    }

    /// Get employee by ID
    pub async fn get_by_id(&self, id: i64) -> Result<EmployeeResponse, AppError> {
        let employee = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Employee not found".to_string()))?;

        Ok(EmployeeResponse::from(employee))
    }

    /// Get all employees with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<EmployeeResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<EmployeeResponse> =
            items.into_iter().map(EmployeeResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update employee
    pub async fn update(
        &self,
        id: i64,
        request: UpdateEmployeeRequest,
    ) -> Result<EmployeeResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Employee not found".to_string()))?;
        let name = request.name.clone();
        // Business rule: check duplicate name if changing
        if name != existing.name {
            if let Some(_) = self
                .repository
                .find_by_name(&name, existing.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "Employee with this name already exists".to_string(),
                ));
            }
        }
        // Build update model
        let mut active_model = employees::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        active_model.name = Set(request.name);
        active_model.unit_id = Set(request.unit_id);
        active_model.employee_number = Set(request.employee_number);
        active_model.nik = Set(request.nik);
        active_model.position_id = Set(request.position_id);
        active_model.department_id = Set(request.department_id);
        active_model.employment_type = Set(request.employment_type);
        active_model.hire_date = Set(request.hire_date);
        active_model.salary = Set(request.salary);

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(EmployeeResponse::from(updated))
    }

    /// Delete employee
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Employee not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
