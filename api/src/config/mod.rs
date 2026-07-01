// api/src/config/mod.rs
pub mod app;
pub mod database; // ← tambahkan ini

pub use app::AppConfig;
pub use database::Database;
