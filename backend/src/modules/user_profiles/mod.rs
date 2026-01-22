// backend/src/modules/user_profiles/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::UserProfileRepository;
// pub use routes::configure;
pub use service::UserProfileService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> UserProfileService {
    let repository = UserProfileRepository::new(db);
    UserProfileService::new(repository)
}
// File: backend/src/modules/user_profiles/repository.rs
