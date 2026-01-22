// backend/src/modules/foundations/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::FoundationRepository;
// pub use routes::configure;
pub use service::FoundationService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> FoundationService {
    let repository = FoundationRepository::new(db);
    FoundationService::new(repository)
}
// File: backend/src/modules/foundations/repository.rs
