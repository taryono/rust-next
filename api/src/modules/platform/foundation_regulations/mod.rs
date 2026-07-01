// api/src/modules/foundation_regulations/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::FoundationRegulationRepository;
// pub use routes::configure;
pub use service::FoundationRegulationService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> FoundationRegulationService {
    let repository = FoundationRegulationRepository::new(db);
    FoundationRegulationService::new(repository)
}
// File: api/src/modules/foundation_regulations/repository.rs
