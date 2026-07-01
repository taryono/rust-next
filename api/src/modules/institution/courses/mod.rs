// api/src/modules/classes/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::CourseRepository;
// pub use routes::configure;
pub use service::CourseService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> CourseService {
    let repository = CourseRepository::new(db);
    CourseService::new(repository)
}
// File: api/src/modules/classes/repository.rs
