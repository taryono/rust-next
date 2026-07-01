// api/src/modules/payments/mod.rs
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
pub use repository::PaymentRepository;
// pub use routes::configure;
pub use service::PaymentService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> PaymentService {
    let repository = PaymentRepository::new(db);
    PaymentService::new(repository)
}
// File: api/src/modules/payments/repository.rs
