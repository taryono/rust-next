// backend/src/modules/attendances/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::AttendanceRepository;
// pub use routes::configure;
pub use service::AttendanceService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> AttendanceService {
    let repository = AttendanceRepository::new(db);
    AttendanceService::new(repository)
}
// File: backend/src/modules/attendances/repository.rs
