// api/src/modules/book_copies/mod.rs
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
pub use repository::BookCopiesRepository;
// pub use routes::configure;
pub use service::BookCopiesService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> BookCopiesService {
    let repository = BookCopiesRepository::new(db);
    BookCopiesService::new(repository)
}
// File: api/src/modules/book_copies/repository.rs
