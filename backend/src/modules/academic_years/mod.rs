// backend/src/modules/academic_years/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::AcademicYearRepository;
// pub use routes::configure;
pub use service::AcademicYearService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> AcademicYearService {
    let repository = AcademicYearRepository::new(db);
    AcademicYearService::new(repository)
}
// File: backend/src/modules/academic_years/repository.rs
