// src/app_state.rs

use crate::modules::academic_years::AcademicYearService;

use std::sync::Arc;
pub struct AppState {
    pub academic_year_service: Arc<AcademicYearService>,
}
impl AppState {
    pub fn new(academic_year_service: AcademicYearService) -> Self {
        Self {
            academic_year_service: Arc::new(academic_year_service),
        }
    }
}
