// api/src/modules/publisher/mod.rs
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
pub use repository::PublisherRepository;
// pub use routes::configure;
pub use service::PublisherService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> PublisherService {
    let repository = PublisherRepository::new(db);
    PublisherService::new(repository)
}
// File: api/src/modules/publisher/repository.rs
