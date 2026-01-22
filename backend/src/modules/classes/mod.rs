// backend/src/modules/classes/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::ClassRepository;
// pub use routes::configure;
pub use service::ClassService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> ClassService {
    let repository = ClassRepository::new(db);
    ClassService::new(repository)
}
// File: backend/src/modules/classes/repository.rs
