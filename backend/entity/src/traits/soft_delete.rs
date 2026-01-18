// entity/src/traits/soft_delete.rs
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

/// Trait untuk entity yang support soft delete
/// Hanya untuk query helpers, implementasi soft delete ada di service layer
pub trait SoftDelete: EntityTrait {
    /// Column untuk deleted_at
    fn deleted_at_col() -> Self::Column;

    /// Query hanya record yang belum dihapus (exclude soft deleted)
    fn find_not_deleted() -> sea_orm::Select<Self> {
        Self::find().filter(Self::deleted_at_col().is_null())
    }

    /// Query hanya record yang sudah dihapus (only soft deleted)
    fn find_only_deleted() -> sea_orm::Select<Self> {
        Self::find().filter(Self::deleted_at_col().is_not_null())
    }

    /// Query semua record (include soft deleted)
    fn find_with_deleted() -> sea_orm::Select<Self> {
        Self::find()
    }
}
