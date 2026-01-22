// backend/src/modules/permissions/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::PermissionRepository;
// pub use routes::configure;
pub use service::PermissionService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> PermissionService {
    let repository = PermissionRepository::new(db);
    PermissionService::new(repository)
}
// File: backend/src/modules/permissions/repository.rs
