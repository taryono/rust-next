// backend/src/modules/rooms/mod.rs
// ============================================================================
// mod.rs - Module Entry Point
// ============================================================================
pub mod docs;
pub mod dto;
pub mod handler;
pub mod repository;
pub mod routes;
pub mod service;

pub use repository::RoomRepository;
// pub use routes::configure;
pub use service::RoomService;

// Helper untuk initialize service dengan dependencies
use crate::config::database::Database;

pub fn init_service(db: Database) -> RoomService {
    let repository = RoomRepository::new(db);
    RoomService::new(repository)
}
// File: backend/src/modules/rooms/repository.rs
