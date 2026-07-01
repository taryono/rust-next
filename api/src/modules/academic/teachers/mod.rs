// api/src/modules/academic/teachers/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::TeacherRepository;
// pub use routes::configure;
pub use service::TeacherService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> TeacherService {
    let repository = TeacherRepository::new(db);
    TeacherService::new(repository)
}
// File: api/src/modules/academic/teachers/repository.rs
