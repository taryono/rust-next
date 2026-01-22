// backend/src/modules/positions/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::PositionRepository;
// pub use routes::configure;
pub use service::PositionService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> PositionService {
    let repository = PositionRepository::new(db);
    PositionService::new(repository)
}
// File: backend/src/modules/positions/repository.rs
