// entity/src/traits/tenanted.rs
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select};

/// Trait untuk model multi-tenant berbasis foundation_id
pub trait Tenanted: EntityTrait {
    type Col: ColumnTrait;

    /// Kembalikan kolom yang menyimpan foundation_id
    fn foundation_id_column() -> Self::Col;

    /// Filter data hanya milik foundation tertentu
    fn for_foundation(foundation_id: i64) -> Select<Self> {
        Self::find().filter(Self::foundation_id_column().eq(foundation_id))
    }
}
