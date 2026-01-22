// backend/src/modules/departments/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::DepartmentRepository;
// pub use routes::configure;
pub use service::DepartmentService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> DepartmentService {
    let repository = DepartmentRepository::new(db);
    DepartmentService::new(repository)
}
// File: backend/src/modules/departments/repository.rs
