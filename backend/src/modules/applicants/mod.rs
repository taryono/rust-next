// backend/src/modules/applicants/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::ApplicantRepository;
// pub use routes::configure;
pub use service::ApplicantService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> ApplicantService {
    let repository = ApplicantRepository::new(db);
    ApplicantService::new(repository)
}
// File: backend/src/modules/applicants/repository.rs
