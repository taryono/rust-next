// backend/src/modules/semesters/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::SemesterRepository;
// pub use routes::configure;
pub use service::SemesterService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> SemesterService {
    let repository = SemesterRepository::new(db);
    SemesterService::new(repository)
}
// File: backend/src/modules/semesters/repository.rs
