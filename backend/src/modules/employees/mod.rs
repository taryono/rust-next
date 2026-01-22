// backend/src/modules/employees/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::EmployeeRepository;
// pub use routes::configure;
pub use service::EmployeeService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> EmployeeService {
    let repository = EmployeeRepository::new(db);
    EmployeeService::new(repository)
}
// File: backend/src/modules/employees/repository.rs
