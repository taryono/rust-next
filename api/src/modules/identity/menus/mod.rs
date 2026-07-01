// api/src/modules/menus/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::MenuRepository;
// pub use routes::configure;
pub use service::MenuService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> MenuService {
    let repository = MenuRepository::new(db);
    MenuService::new(repository)
}
// File: api/src/modules/menus/repository.rs
