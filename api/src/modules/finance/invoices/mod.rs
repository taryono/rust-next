// api/src/modules/invoices/mod.rs
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
pub use repository::InvoiceRepository;
// pub use routes::configure;
pub use service::InvoiceService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> InvoiceService {
    let repository = InvoiceRepository::new(db);
    InvoiceService::new(repository)
}
// File: api/src/modules/invoices/repository.rs
