// backend/src/modules/class_levels/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::ClassLevelRepository;
// pub use routes::configure;
pub use service::ClassLevelService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> ClassLevelService {
    let repository = ClassLevelRepository::new(db);
    ClassLevelService::new(repository)
}
// File: backend/src/modules/class_levels/repository.rs
