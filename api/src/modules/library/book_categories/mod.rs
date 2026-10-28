// api/src/modules/library/book_categories/mod.rs
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
pub use repository::BookCategoryRepository;
pub use service::BookCategoryService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> BookCategoryService {
    let repository = BookCategoryRepository::new(db);
    BookCategoryService::new(repository)
}
