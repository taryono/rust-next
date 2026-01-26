pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::UserRepository;
// pub use routes::configure;
pub use service::UserService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> UserService {
    let repository = UserRepository::new(db);
    UserService::new(repository)
}
// File: backend/src/modules/employees/repository.rs
