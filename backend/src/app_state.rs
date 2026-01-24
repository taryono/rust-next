// src/app_state.rs
use crate::modules::academic_years::AcademicYearService;
use crate::modules::auth::AuthService;
use crate::modules::employees::EmployeeService;
use crate::modules::permissions::PermissionService;
use crate::modules::positions::PositionService;
use std::sync::Arc;
pub struct AppState {
    pub academic_year_service: Arc<AcademicYearService>,
    pub auth_service: Arc<AuthService>, // ← Tambahkan ini
    pub permission_service: Arc<PermissionService>,
    pub position_service: Arc<PositionService>,
    pub employee_service: Arc<EmployeeService>,
}
impl AppState {
    pub fn new(
        academic_year_service: AcademicYearService,
        auth_service: AuthService,
        permission_service: PermissionService,
        position_service: PositionService,
        employee_service: EmployeeService,
    ) -> Self {
        Self {
            academic_year_service: Arc::new(academic_year_service),
            auth_service: Arc::new(auth_service),
            permission_service: Arc::new(permission_service),
            position_service: Arc::new(position_service),
            employee_service: Arc::new(employee_service),
        }
    }
}
