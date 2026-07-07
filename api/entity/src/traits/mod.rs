// backend/entity/src/traits/mod.rs
pub mod soft_delete;
pub mod tenanted;
pub use soft_delete::SoftDelete;
pub use tenanted::Tenanted;
