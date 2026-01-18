// src/macros/soft_delete.rs
#[macro_export]
macro_rules! impl_soft_delete {
    ($entity:ty, $column:expr) => {
        impl $crate::traits::soft_delete::SoftDelete for $entity {
            fn deleted_at_col() -> <Self as sea_orm::EntityTrait>::Column {
                $column
            }
        }
    };
}

// Penggunaan di entity
// impl_soft_delete!(users::Entity, users::Column::DeletedAt);
// impl_soft_delete!(posts::Entity, posts::Column::DeletedAt);
// impl_soft_delete!(comments::Entity, comments::Column::DeletedAt);
