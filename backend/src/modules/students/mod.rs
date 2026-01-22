// backend/src/modules/students/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::StudentRepository;
// pub use routes::configure;
pub use service::StudentService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> StudentService {
    let repository = StudentRepository::new(db);
    StudentService::new(repository)
}
// File: backend/src/modules/students/repository.rs
