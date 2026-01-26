// backend/src/modules/roles/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::RoleRepository;
// pub use routes::configure;
pub use service::RoleService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> RoleService {
    let repository = RoleRepository::new(db);
    RoleService::new(repository)
}
// File: backend/src/modules/roles/repository.rs
