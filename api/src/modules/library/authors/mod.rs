// api/src/modules/author/mod.rs
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
pub use repository::BookRepository;
// pub use routes::configure;
pub use service::AuthorService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> AuthorService {
    let repository = BookRepository::new(db);
    AuthorService::new(repository)
}
// File: api/src/modules/author/repository.rs
