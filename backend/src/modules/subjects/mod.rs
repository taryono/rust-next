// backend/src/modules/subjects/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::SubjectRepository;
// pub use routes::configure;
pub use service::SubjectService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> SubjectService {
    let repository = SubjectRepository::new(db);
    SubjectService::new(repository)
}
// File: backend/src/modules/subjects/repository.rs
