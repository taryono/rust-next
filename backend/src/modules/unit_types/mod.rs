// backend/src/modules/unit_types/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::UnitTypeRepository;
// pub use routes::configure;
pub use service::UnitTypeService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> UnitTypeService {
    let repository = UnitTypeRepository::new(db);
    UnitTypeService::new(repository)
}
// File: backend/src/modules/unit_types/repository.rs
