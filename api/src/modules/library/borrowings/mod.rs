// api/src/modules/borrowings/mod.rs
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
pub use repository::BorrowingRepository;
// pub use routes::configure;
pub use service::BorrowingService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> BorrowingService {
    let repository = BorrowingRepository::new(db);
    BorrowingService::new(repository)
}
// File: api/src/modules/borrowings/repository.rs
