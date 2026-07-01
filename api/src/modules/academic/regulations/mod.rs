// api/src/modules/academic/regulations/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::RegulationRepository;
// pub use routes::configure;
pub use service::RegulationService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> RegulationService {
    let repository = RegulationRepository::new(db);
    RegulationService::new(repository)
}
// File: api/src/modules/academic/regulations/repository.rs
