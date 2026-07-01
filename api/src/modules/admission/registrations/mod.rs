// api/src/modules/registrations/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::RegistrationRepository;
// pub use routes::configure;
pub use service::RegistrationService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> RegistrationService {
    let repository = RegistrationRepository::new(db);
    RegistrationService::new(repository)
}
// File: api/src/modules/registrations/repository.rs
