// api/src/modules/guardians/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod searchable;
pub mod service;
pub use repository::GuardianRepository;
// pub use routes::configure;
pub use service::GuardianService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> GuardianService {
    let repository = GuardianRepository::new(db);
    GuardianService::new(repository)
}
// File: api/src/modules/guardians/repository.rs
