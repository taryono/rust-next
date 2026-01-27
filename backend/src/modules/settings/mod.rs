// backend/src/modules/settings/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::SettingRepository;
// pub use routes::configure;
pub use service::SettingService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> SettingService {
    let repository = SettingRepository::new(db);
    SettingService::new(repository)
}
// File: backend/src/modules/settings/repository.rs
