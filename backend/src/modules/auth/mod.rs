// src/modules/auth/mod.rs
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

// Re-export commonly used items
pub use repository::AuthRepository;
pub use service::AuthService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> AuthService {
    let repository = AuthRepository::new(db);
    AuthService::new(repository)
}
// File: backend/src/modules/academic_years/repository.rs
