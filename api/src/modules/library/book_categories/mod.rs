// api/src/modules/categories/mod.rs
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
pub use service::BookCategoryService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> BookCategoryService {
    let repository = BookRepository::new(db);
    BookCategoryService::new(repository)
}
// File: api/src/modules/categories/repository.rs
