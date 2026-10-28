// api/src/modules/library/authors/mod.rs
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
pub use repository::AuthorRepository;
pub use service::AuthorService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> AuthorService {
    let repository = AuthorRepository::new(db);
    AuthorService::new(repository)
}
